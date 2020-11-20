use std::io::{self, Write};
use std::error::Error;
use clap::Clap;
use i3ipc::I3EventListener;
use i3ipc::I3Connection;
use i3ipc::Subscription;
use i3ipc::reply::Node;
use i3ipc::event::Event;
use i3ipc::event::inner::WindowChange;

/// prints the title of the focused i3 container
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Kacper W. <kacper@wardega.com>")]
struct Opts {
    /// Truncate the output to a length > 3. A value of zero means do not truncate.
    #[clap(short, long, default_value = "0", validator = check_truncate)]
    truncate: usize,

    /// Subscribe to i3 events. If set, i3title prints container titles
    /// line-by-line as the focus changes.
    #[clap(short, long)]
    subscribe: bool,
}

fn check_truncate(v: &str) -> Result<(), String> {
    if (v == "1") | (v == "2") | (v == "3") { return Err("Acceptable values are 0, 4, 5, ...".to_string()); }
    Ok(())
}

fn find_focused(node: &Node) -> Option<&Node> {
    if node.focused {
        Some(node)
    } else {
        // it's not clear to me if nodes or floating_nodes are also sorted in focus order
        // so just linear scan?
        for child in &node.nodes {
            if child.id == node.focus[0] {
                return find_focused(&child);
            }
        }
        for child in &node.floating_nodes {
            if child.id == node.focus[0] {
                return find_focused(&child);
            }
        }
        None
    }
}

fn print_name(name: &String, truncate: usize) {
    let stdout = io::stdout();
    {
        let mut handle = stdout.lock();
        if name.chars().count() > truncate {
            write!(handle, "{}...\n", &name.chars().take(truncate-3).collect::<String>()).ok();
        } else {
            write!(handle, "{}\n", name).ok();
        }
    } // handle goes out of scope, flushing stdout
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let print_length = if opts.truncate == 0 { usize::MAX } else { opts.truncate };

    let mut connection = I3Connection::connect()?;
    match find_focused(&connection.get_tree()?) {
        Some(node) => print_name(node.name.as_ref().or(Some(&"?...".to_string())).unwrap(), print_length),
        // could not find initially focused container
        _ => print_name(&"?...".to_string(), print_length),
    };

    if !opts.subscribe { return Ok(()); }

    let mut listener = I3EventListener::connect()?;
    listener.subscribe(&[Subscription::Window])?;
    for event in listener.listen() {
        match event? {
            Event::WindowEvent(e) => match e.change {
                WindowChange::Focus => if let Some(name) = e.container.name {
                    print_name(&name, print_length);
                },
                WindowChange::Title => if let true = e.container.focused {
                    if let Some(name) = e.container.name {
                        print_name(&name, print_length);
                    }
                }
                _ => (),
            },
            _ => unreachable!(),
        }
    }
    Ok(())
}
