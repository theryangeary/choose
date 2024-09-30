use std::convert::TryInto;

use crate::config::Config;
use crate::error::Error;
use crate::result::Result;
use crate::writeable::Writeable;
use crate::writer::WriteReceiver;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Choice {
    pub start: isize,
    pub end: isize,
    pub kind: ChoiceKind,
    negative_index: bool,
    reversed: bool,
}

#[derive(Debug, PartialEq)]
pub enum ChoiceKind {
    Single,
    RustExclusiveRange,
    RustInclusiveRange,
    ColonRange,
}

impl Choice {
    pub fn new(start: isize, end: isize, kind: ChoiceKind) -> Self {
        let negative_index = start < 0 || end < 0;
        let reversed = end < start && !(start >= 0 && end < 0);
        Choice {
            start,
            end,
            kind,
            negative_index,
            reversed,
        }
    }

    pub fn print_choice<W: WriteReceiver>(
        &self,
        line: &str,
        config: &Config,
        handle: &mut W,
    ) -> Result<()> {
        if config.opt.character_wise {
            self.print_choice_generic(line.chars(), config, handle)
        } else {
            let line_iter = config
                .separator
                .split(line)
                .filter(|s| !s.is_empty() || config.opt.non_greedy);
            self.print_choice_generic(line_iter, config, handle)
        }
    }

    pub fn is_reverse_range(&self) -> bool {
        self.reversed
    }

    pub fn has_negative_index(&self) -> bool {
        self.negative_index
    }

    fn print_choice_generic<W, T, I>(
        &self,
        mut iter: I,
        config: &Config,
        handle: &mut W,
    ) -> Result<()>
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        if self.is_reverse_range() && !self.has_negative_index() {
            self.print_choice_reverse(iter, config, handle)?;
        } else if self.has_negative_index() {
            self.print_choice_negative(iter, config, handle)?;
        } else {
            if self.start > 0 {
                iter.nth((self.start - 1).try_into()?);
            }
            let range = self
                .end
                .checked_sub(self.start)
                .ok_or_else(|| Error::Config("expected end > start".into()))?;
            Choice::print_choice_loop_max_items(iter, config, handle, range)?;
        }

        Ok(())
    }

    fn print_choice_loop_max_items<W, T, I>(
        iter: I,
        config: &Config,
        handle: &mut W,
        max_items: isize,
    ) -> Result<()>
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        let mut peek_iter = iter.peekable();
        for i in 0..=max_items {
            match peek_iter.next() {
                Some(s) => {
                    handle.write_choice(s, config, peek_iter.peek().is_some() && i != max_items)?;
                }
                None => break,
            };
        }

        Ok(())
    }

    /// Print choices that include at least one negative index
    fn print_choice_negative<W, T, I>(&self, iter: I, config: &Config, handle: &mut W) -> Result<()>
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        let vec = iter.collect::<Vec<_>>();

        if let Some((start, end)) = self.get_negative_start_end(&vec)? {
            if end > start {
                for word in vec[start..std::cmp::min(end, vec.len() - 1)].iter() {
                    handle.write_choice(*word, config, true)?;
                }
                handle.write_choice(vec[std::cmp::min(end, vec.len() - 1)], config, false)?;
            } else if self.start < 0 {
                for word in vec[end + 1..=std::cmp::min(start, vec.len() - 1)]
                    .iter()
                    .rev()
                {
                    handle.write_choice(*word, config, true)?;
                }
                handle.write_choice(vec[end], config, false)?;
            } else if start == end && self.start < vec.len().try_into()? {
                handle.write_choice(vec[start], config, false)?;
            }
        }

        Ok(())
    }

    fn print_choice_reverse<W, T, I>(
        &self,
        mut iter: I,
        config: &Config,
        handle: &mut W,
    ) -> Result<()>
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        if self.end > 0 {
            iter.nth((self.end - 1).try_into()?);
        }

        let mut stack = Vec::new();
        for i in 0..=(self.start - self.end) {
            match iter.next() {
                Some(s) => stack.push(s),
                None => break,
            }

            if self.start <= self.end + i {
                break;
            }
        }

        let mut peek_iter = stack.iter().rev().peekable();
        while let Some(s) = peek_iter.next() {
            handle.write_choice(*s, config, peek_iter.peek().is_some())?;
        }

        Ok(())
    }

    /// Get the absolute indexes of a choice range based on the slice length
    ///
    /// N.B. that this assumes that at least one index is negative - do not try to call this
    /// function with a purely positive range.
    ///
    /// Returns Ok(None) if the resulting choice range would not include any item in the slice.
    fn get_negative_start_end<T>(&self, slice: &[T]) -> Result<Option<(usize, usize)>> {
        if slice.is_empty() {
            return Ok(None);
        }

        let start_abs = self.start.checked_abs().ok_or_else(|| {
            Error::Config(format!(
                "Minimum index value supported is isize::MIN + 1 ({})",
                isize::MIN + 1
            ))
        })?;

        let slice_len_as_isize = slice.len().try_into()?;

        if self.kind == ChoiceKind::Single {
            if start_abs <= slice_len_as_isize {
                let idx = (slice_len_as_isize - start_abs).try_into()?;
                Ok(Some((idx, idx)))
            } else {
                Ok(None)
            }
        } else {
            let end_abs = self.end.checked_abs().ok_or_else(|| {
                Error::Config(format!(
                    "Minimum index value supported is isize::MIN + 1 ({})",
                    isize::MIN + 1
                ))
            })?;

            if self.start >= 0 {
                // then we assume self.end is negative
                let start = self.start.try_into()?;

                if end_abs <= slice_len_as_isize || start <= slice.len() || start > slice.len() {
                    let end = slice.len().saturating_sub(end_abs.try_into()?);
                    Ok(Some((
                        std::cmp::min(start, slice.len().saturating_sub(1)),
                        std::cmp::min(end, slice.len().saturating_sub(1)),
                    )))
                } else {
                    Ok(None)
                }
            } else if self.end >= 0 {
                // then we assume self.start is negative
                let end = self.end.try_into()?;

                if start_abs <= slice_len_as_isize || end <= slice.len() || end > slice.len() {
                    let start = slice.len().saturating_sub(start_abs.try_into()?);
                    Ok(Some((
                        std::cmp::min(start, slice.len().saturating_sub(1)),
                        std::cmp::min(end, slice.len().saturating_sub(1)),
                    )))
                } else {
                    Ok(None)
                }
            } else {
                // both indices are negative
                let start = slice.len().saturating_sub(start_abs.try_into()?);
                let end = slice.len().saturating_sub(end_abs.try_into()?);

                if start_abs <= slice_len_as_isize || end_abs <= slice_len_as_isize {
                    Ok(Some((
                        std::cmp::min(start, slice.len().saturating_sub(1)),
                        std::cmp::min(end, slice.len().saturating_sub(1)),
                    )))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
