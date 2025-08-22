fn main() {

    const COIN_HRSH :u64 = 320;
    const USDC :u64 = 3200;
    const SWAP  :u64= 495;

    const COIN_APT:u64 = 340;
    const USDC_APT:u64 = 6000;

    const COIN_ETH:u64 = 500;
    const USDC_ETH :u64 = 1200;

    const HRSH :u64 = 1;
    const APT :u64 = 2;
    const ETH :u64= 3;

    fn multi(coin_type:u64)->(u64 , u64){
    if coin_type == HRSH {
        return (USDC , COIN_HRSH);
    }
    else if coin_type == APT {
        return (USDC_APT,COIN_APT)
    }
    else {
        return (USDC_ETH , COIN_APT)
    }
}
    let (coin1 , coin2) = multi(ETH);

    let result = calculate_swap(coin1 , coin2 , SWAP);
    println!("{}",result);
    
}

fn calculate_swap(coin1 :u64 , coin2 :u64 , coin_amount:u64) -> u64{
    let fees = coin_amount * 5 / 1000;
    let min_supply = coin1 * coin2;
    let new_usdc = coin1 + coin_amount;
    let new_hrsh = min_supply / (new_usdc - fees); 
    let recieved = coin2 - new_hrsh;
    return recieved;
}

