use std::{collections::BTreeMap, fs::File, io::{BufRead, BufReader, Write}};

use tempfile::tempdir;

pub fn billion_row_challenge() {
    let data = r#"Tokyo;35.6897
Jakarta;-6.1750
Delhi;28.6100
Guangzhou;23.1300
Mumbai;19.0761
Manila;14.5958
Shanghai;31.1667
São Paulo;-23.5500
Seoul;37.5600
Mexico City;19.4333
Cairo;30.0444
New York;40.6943
Dhaka;23.7639
Beijing;39.9040
Kolkāta;22.5675
Bangkok;13.7525
Shenzhen;22.5350
Moscow;55.7558
Buenos Aires;-34.5997
Lagos;6.4550
Istanbul;41.0136
Karachi;24.8600
Bangalore;12.9789
Ho Chi Minh City;10.7756
Ōsaka;34.6939
Chengdu;30.6600
Tehran;35.6892
Kinshasa;-4.3250
Rio de Janeiro;-22.9111
Chennai;13.0825
Xi’an;34.2667
Lahore;31.5497
Chongqing;29.5500
Los Angeles;34.1141
Baoding;38.8671
London;51.5072
Paris;48.8567
Linyi;35.1041
Dongguan;23.0475
Hyderābād;17.3850
Tianjin;39.1467
Lima;-12.0600
Wuhan;30.5872
Nanyang;32.9987
Hangzhou;30.2500
Foshan;23.0292
Nagoya;35.1833
Taipei;25.0375
Tongshan;34.2610
Luanda;-8.8383
Zhoukou;33.6250
Ganzhou;25.8292
Kuala Lumpur;3.1478
Heze;35.2333
Quanzhou;24.9139
Chicago;41.8375
Nanjing;32.0608
Jining;35.4000
Hanoi;21.0283
Pune;18.5203
Fuyang;32.8986
Ahmedabad;23.0300
Johannesburg;-26.2044
Bogotá;4.7111
Dar es Salaam;-6.8161
Shenyang;41.8025
Khartoum;15.5006
Shangqiu;34.4259
Cangzhou;38.3037
Hong Kong;22.3000
Shaoyang;27.2418
Zhanjiang;21.1967
Yancheng;33.3936
Hengyang;26.8968
Riyadh;24.6333
Zhumadian;32.9773
Santiago;-33.4372
Xingtai;37.0659
Chattogram;22.3350
Bijie;27.3019
Shangrao;28.4419
Zunyi;27.7050
Sūrat;21.1702
Surabaya;-7.2458
Huanggang;30.4500
Maoming;21.6618
Nanchong;30.7991
Xinyang;32.1264
Madrid;40.4169
Baghdad;33.3153
Qujing;25.5102
Jieyang;23.5533
Singapore;1.3000
Prayagraj;25.4358
Liaocheng;36.4500
Dalian;38.9000
Yulin;22.6293
Changde;29.0397
Qingdao;36.1167
Douala;4.0500"#;
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("weather_stations.csv");
    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    } 
    let file_path = file_path.to_str().unwrap();
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut stats = BTreeMap::<String, (f64, f64, usize, f64)>::new(); // min, sum, count, max

    for line in reader.lines() {
        dbg!(&line);
        let line = line.unwrap();
        let (name, temp) = line.split_once(";").unwrap();
        let temp = temp.parse::<f64>().unwrap();
        let stats = stats.entry(name.to_string()).or_insert((f64::MAX, 0.0, 0, f64::MIN));
        stats.0 = stats.0.min(temp);
        stats.1 += temp;
        stats.2 += 1;
        stats.3 = stats.3.max(temp);
    }

    for (stat, (min, sum, count, max)) in stats {
        println!("{stat}={min:.1}/{:.1}/{max:.1}", sum / (count as f64));
    }
}
