fn main() {
    let uc = TickerPricerUseCase::new(Repository::MongoDB);
    uc.execute();
}

struct TickerPricerUseCase {
    repository: Repository
}

impl TickerPricerUseCase {
    fn new(repository: Repository) -> Self {
        Self {
            repository
        }
    }

    fn execute(self) {
        match self.repository {
            Repository::MongoDB => println!("Hey !"),
        }
        println!("No reduction for you");
    }
}

enum Repository {
    MongoDB
}
