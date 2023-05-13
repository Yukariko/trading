use crate::strategy::*;
use crate::Session;
use crate::database::DataBase;

pub struct TimeRunner<'a> {
    session: Session,
    database: DataBase<'a>,
}

impl<'a> TimeRunner<'a> {
    pub fn new(session: Session) -> TimeRunner<'a> {
        TimeRunner {
            session: session,
            database: DataBase::new(),
        }
    }

    pub fn run(&self, start_date: u32, end_date: u32, strategies: Vec<Box<dyn Strategy>>) {

        let mut iters: Vec<Box<StrategyIterator>> = strategies
            .iter()
            .map(
                |strategy| strategy.iter(
                    start_date,
                    &self.database)
            )
            .collect();

        for date in start_date..=end_date {
            for strategy_iter in &mut iters {
                if let Some(commands) = strategy_iter.next() {
                    //let res = self.session.execute_vec(commands)

                }
            }
        }
    }
}
