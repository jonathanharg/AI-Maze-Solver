use crate::coord::Coord;
use crate::maze::{Maze, Tile};
use crossterm::style::{style, Color, Stylize};
use crossterm::{cursor, style, terminal, QueueableCommand};
use std::io::{stdout, Write};

pub fn full(maze: &Maze) -> crossterm::Result<()> {
    // let width: u16 = maze.width.try_into().unwrap_or(75).clamp(0, 75);
    // let height: u16 = maze.height.try_into().unwrap_or(20).clamp(0, 20);
    let width: u16 = maze.width.try_into().unwrap_or(200).clamp(0, 200);
    let height: u16 = maze.height.try_into().unwrap_or(50).clamp(0, 50);
    let mut stdout = stdout();
    for y in 0..height {
        for x in 0..width {
            let coord = (x.into(), y.into()).into();
            let cell = maze.get(coord).unwrap();
            stdout.queue(cursor::MoveTo(x, y))?;
            match cell {
                Tile::Wall => {
                    stdout
                        // .queue(style::SetBackgroundColor(Color::Black))?
                        // .queue(style::PrintStyledContent('#'.black()))?;
                        .queue(style::Print('#'))?;
                }
                Tile::Path => {
                    // if visited.contains(&coord) {
                    //     stdout
                    //         .queue(style::SetBackgroundColor(Color::Red))?
                    //         .queue(style::Print('-'))?;
                    // } else {
                    stdout
                        // .queue(style::SetBackgroundColor(Color::White))?
                        // .queue(style::PrintStyledContent('-'.white()))?;
                        .queue(style::Print('-'))?;
                    // }
                }
            };
        }
    }
    stdout.queue(style::Print('\n'))?;
    stdout.queue(cursor::SavePosition)?;
    stdout.flush()?;
    Ok(())
}

pub fn visit(coord: &Coord) -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(
        coord.x.try_into().unwrap(),
        coord.y.try_into().unwrap(),
    ))?;
    stdout.queue(style::Print('x'))?;
    stdout.flush()?;
    Ok(())
}

pub fn clear_terminal() -> crossterm::Result<()> {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::Hide)?
        .queue(terminal::DisableLineWrap)?
        .flush()
}

pub fn finish() -> crossterm::Result<()> {
    stdout()
        .queue(cursor::Show)?
        .queue(terminal::EnableLineWrap)?
        .flush()
}

