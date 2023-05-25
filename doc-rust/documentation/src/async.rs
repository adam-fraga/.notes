pub mod async_module {
    use tokio::io::AsyncReadExt;
    use tokio::time;

    //Function calcul fib suit (expensive for async demonstration )

    pub fn fib(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            n => fib(n - 1) + fib(n - 2),
        }
    }

    // ---------------------SYNCHRONE VERSION---------------------------

    pub fn read_file_sync() {
        log::info!("Reading some beeg data...");
        let mut file = fs::File::open("Foo.csv").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        log::info!("Reading beeg {} bytes...", content.len());
    }

    pub fn run_sync() {
        for _ in 0..1000 {
            read_file_sync();
        }
    }

    // ---------------------ASYNCHRONE VERSION---------------------------

    // Async fn return a future, future are lazy and are not execute, untill you call await.
    pub async fn basic_async() {
        log::info!("Sleeping");
        time::sleep(time::Duration::from_secs(1)).await;
        log::info!("Awake")
    }

    //Read from a big CSV file, and count the number of byte inside of it.
    pub async fn read_file_async() {
        log::info!("Reading some beeg data...");
        let mut file = tokio::fs::File::open("Foo.csv").await.unwrap();
        let mut content = vec![];
        file.read_to_end(&mut content).await.unwrap();
        log::info!("Reading beeg {} bytes...", content.len());

        //Execure expansive fib calculation to another thread to optimize reading file async
        tokio::task::spawn_blocking(move || {
            log::info!("Computing fib 40");
            fib(40);
        })
        .await
        .unwrap();

        //Uncomment tokio::task::spawn::blocking to visualise difference
        // fib(40);
    }

    pub async fn run() {
        //Join macros run the function you store inside concurrently (in the same time)
        //It's really useful for blocking function with a large execution time.
        tokio::join!(
            basic_async(),
            read_file_async(),
            read_file_async(),
            read_file_async(),
            read_file_async(),
            read_file_async()
        );
    }

    //Spawn provide a way to leave an asynf cunction to execute and continue to the next instructions
    pub async fn fire_and_forget() {
        tokio::spawn(async {
            basic_async().await;
        });
        simple_program().await;
    }
}

// #[tokio::main] => Provide a way to avoid runtime instanciation, transofme main to an async function
fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let future = async_module::basic_async();
    tokio_runtime.block_on(future);
}
