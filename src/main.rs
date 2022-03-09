mod GoldProductionDepartment;
mod GoldSupplyDepartment;
use GoldProductionDepartment::MiningDepartment;
use GoldSupplyDepartment::RoyalBank;
use tui::backend::{self, Backend};

use {
    std::{
        io,
        thread,
        time::Duration
    },
    tui::{
        backend::CrosstermBackend,
        widgets::{
            Widget,
            Block,
            Borders
        },
        layout::{
            Layout,
            Constraint,
            Direction
        },
        Terminal
    },
    crossterm::{
        event::{
            self,
            DisableMouseCapture,
            EnableMouseCapture,
            Event,
            KeyCode
        },
        execute,
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen
        }
    },
};

fn main() {
    match enable_raw_mode() {
        Ok(ok) => (),
        Err(Error) => panic!("{}", Error)
    };

    let mut stdout = io::stdout();
    match execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
        Ok(ok) => (),
        Err(Error) => panic!("{}", Error)
    };
    let backend = CrosstermBackend::new(stdout);
    let mut Terminal = match Terminal::new(backend) {
        Ok(r) => r,
        Err(e) => panic!("{}", e)
    };

    match Terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    }) {
        Ok(r) => (),
        Err(e) => panic!("{}", e)
    };

    thread::sleep(Duration::from_millis(5000));

    match disable_raw_mode() {
        Ok(r) => (),
        Err(e) => panic!("{}", e)
    };
    match execute!(Terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture) {
        Ok(r) => (),
        Err(e) => panic!("{}", e)
    };
    match Terminal.show_cursor() {
        Ok(r) => (),
        Err(e) => panic!("{}", e)
    };

    let mut RB = RoyalBank::new(1.0, 2.0);
    for i in 0..10 {
        let MD = MiningDepartment::new(1.0);
        RB.getGoldOutput(MD.goldOutput());
        println!("current royal bank gold: {}", RB.checkGold());
    }
}
