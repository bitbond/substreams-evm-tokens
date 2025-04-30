use crate::abi::functions;
use common::{bigint_to_uint8, bytes32_to_string};
use substreams_abis::evm::tokens;
use substreams_ethereum::{pb::eth::v2::Call, Function};

pub struct SetNameSymbol {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i32>,
}

pub fn get_metadata<'a>(call: &'a Call) -> Option<SetNameSymbol> {
    // extract both Name & Symbol from the call
    if let Some((name, symbol)) = get_name_symbol(call) {
        return Some(SetNameSymbol {
            name: Some(name),
            symbol: Some(symbol),
            decimals: None,
        });
    }

    // extracted only name from the call
    if let Some(name) = get_name(call) {
        return Some(SetNameSymbol {
            name: Some(name),
            symbol: None,
            decimals: None,
        });
    }

    // extracted only symbol from the call
    if let Some(symbol) = get_symbol(call) {
        return Some(SetNameSymbol {
            name: None,
            symbol: Some(symbol),
            decimals: None,
        });
    }
    None
}

pub fn get_symbol<'a>(call: &'a Call) -> Option<String> {
    if let Some(result) = functions::SetSymbol::match_and_decode(call) {
        return Some(result.symbol);
    }
    if let Some(result) = functions::ChangeSymbol::match_and_decode(call) {
        return Some(result.symbol);
    }
    if let Some(result) = functions::UpdateSymbol::match_and_decode(call) {
        return Some(result.symbol);
    }
    if let Some(result) = functions::SetTokenSymbol::match_and_decode(call) {
        return Some(result.symbol);
    }
    // USDC Circle
    if let Some(result) = tokens::usdc::functions::InitializeV22::match_and_decode(call) {
        return Some(result.new_symbol);
    }
    None
}

pub fn get_name<'a>(call: &'a Call) -> Option<String> {
    if let Some(result) = functions::SetName::match_and_decode(call) {
        return Some(result.name);
    }
    if let Some(result) = functions::ChangeName::match_and_decode(call) {
        return Some(result.name);
    }
    if let Some(result) = functions::UpdateName::match_and_decode(call) {
        return Some(result.name);
    }
    // USDC Circle
    if let Some(result) = tokens::usdc::functions::InitializeV2::match_and_decode(call) {
        return Some(result.new_name);
    }

    // DAI setName(bytes32)
    // MKR: https://etherscan.io/tx/0xac1adc09b9c5fd60edb48ed06fc1bad6fe0f6774eb76eb39eaba40190f88dae7
    // Blocks: 4620855, 4645274
    if let Some(result) = tokens::sai::functions::SetName::match_and_decode(call) {
        return Some(bytes32_to_string(&result.name.to_vec()));
    }
    None
}

pub fn get_name_symbol<'a>(call: &'a Call) -> Option<(String, String)> {
    if let Some(result) = functions::SetMetadata::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    if let Some(result) = functions::SetNameAndSymbol1::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    if let Some(result) = functions::SetNameAndSymbol2::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    if let Some(result) = functions::SetNameAndTicker::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    if let Some(result) = functions::SetTokenInformation::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }

    // BLENDR
    // Method: 0xb104b321
    // Block: 19349253
    // https://etherscan.io/tx/0xc83d0363850e3cbf82d04df615f3d67ccb1274cfb0944a7bd383ad89e5d3f67f
    if let Some(result) = tokens::blendr::functions::Launch::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    None
}

pub fn get_name_symbol_precision<'a>(call: &'a Call) -> Option<(String, String, i32)> {
    // USDC Circle
    if let Some(result) = tokens::usdc::functions::Initialize::match_and_decode(call) {
        if let Some(decimals) = bigint_to_uint8(&result.token_decimals) {
            return Some((result.token_name, result.token_symbol, decimals));
        }
    }

    // Wormhole
    // TO-DO: More validation needed
    // https://github.com/pinax-network/substreams-evm-tokens/issues/28
    // https://github.com/wormhole-foundation/wormhole/blob/9b9c07b4b1474a1526c9a7346dab640e627630bc/ethereum/contracts/bridge/token/TokenImplementation.sol#L17-L25
    if let Some(result) = functions::Initialize::match_and_decode(call) {
        if let Some(decimals) = bigint_to_uint8(&result.decimals) {
            return Some((result.name, result.symbol, decimals));
        }
    }
    None
}
