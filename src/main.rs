use interval_tree::IntervalTree;

#[derive(Debug)]
struct LiquidityPosition {
    pub address: u32,
    pub coin_amount: u32,
    pub pc_amount: u32
}

fn main() {
    let pos_1 = LiquidityPosition {
        address: 0,
        coin_amount: 100,
        pc_amount: 100
    };

    let pos_2 = LiquidityPosition {
        address: 1,
        coin_amount: 200,
        pc_amount: 300
    };

    let mut tree = IntervalTree::new();

    tree.insert(10..=100, pos_1);
    tree.insert(40..=80, pos_2);

    let mut coin_amount_0 = 0;
    for entry in tree.find(50..=50) {
        println!("{:?} => {:?}", entry.key(), entry.get());
        coin_amount_0 += entry.get().coin_amount;
    }

    println!("Coin amount at 50: {}", coin_amount_0);
}