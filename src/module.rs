use crate::{FLIGHTS_BYTES, FLIGHTS_DATA, ICAO_BYTES, ICAO_DATA, IcaoData, FlightsData};

// スクリプト対策 (in lib::search)
pub(crate) fn escape(content: &str) -> String {
    let mut escaped = String::new();

    let replaced_content = content.replace("　", "").replace(" ", "");

    for c in replaced_content.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }

    escaped
}

// 3or 4文字以内かつアルファベットであるかの確認 (in input_check)
fn format_check(input: &str) -> Result<String, String> {
    if (input.len() == 3 || input.len() == 4) && input.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(input.to_string())
    } else {
        Err("<p>ICAOもしくはIATAで入力してください</p>".to_string())
    }
}

// inputが適切かどうか確認 (in lib::search)
pub(crate) fn input_check(deperture: &str, arrival: &str) -> Result<(String, String), String> {
    // 入力確認処理
    if deperture.is_empty() && arrival.is_empty() {
        Err("<p>空港名を入力してください</p>".to_string())
    } else if deperture.is_empty() {
        Err("<p>出発空港を入力してください</p>".to_string())
    } else if arrival.is_empty() {
        Err("<p>到着空港を入力してください</p>".to_string())
    } else {
        let f_deperture = format_check(deperture)?;
        let f_arrival = format_check(arrival)?;

        Ok((f_deperture, f_arrival))
    }
}

// main下準備
fn get_icao() -> &'static Vec<IcaoData> {
    ICAO_DATA.get_or_init(|| {
        postcard::from_bytes(ICAO_BYTES).expect("ICAOデータ取得で問題が発生しました")
    })
}

fn get_flights() -> &'static Vec<FlightsData> {
    FLIGHTS_DATA.get_or_init(|| {
        postcard::from_bytes(FLIGHTS_BYTES).expect("NUMBERデータ取得で問題が発生しました")
    })
}

fn airport(input: &str) -> Result<String, String> {
    let icao_data = get_icao();

    match icao_data.binary_search_by(|data| data.input.cmp(input)) {
        Ok(found) => Ok(icao_data[found].icao.to_string()),
        Err(_) => Err("入力エラー".to_string()),
    }
}

// main
pub(crate) fn c_core(deperture: &str, arrival: &str) -> String {
    let flights = get_flights();

    // 入力消滅->DBの文字に変換
    let (icao_dep, icao_arr) = match (airport(deperture), airport(arrival)) {
        (Ok(d), Ok(a)) => (d, a),
        (Err(_), Ok(_)) => {
            return "<p>入力された出発空港は就航リストにありませんでした</p>".to_string();
        }
        (Ok(_), Err(_)) => {
            return "<p>入力された到着空港は就航リストにありませんでした</p>".to_string();
        }
        (Err(_), Err(_)) => {
            return "<p>出発空港と到着空港のどちらも就航リストにありませんでした</p>".to_string();
        }
    };

    let mut res = String::new();

    match flights.binary_search_by(|data| (data.dep.as_str(), data.arr.as_str()).cmp(&(&icao_dep, &icao_arr))) {
        Ok(found) => {
            let flight = &flights[found];
            let flight_len = flight.num.len();
            res.push_str(&format!(r#"<p>ボタンを押してプランを作成（SimBrief）</p>
                <p>フライトが{}件見つかりました<br></p>"#, flight_len));
            for num in &flight.num {
                res.push_str(&format!(
                        r#"<div class="result_a_frame">
                            <a href="https://dispatch.simbrief.com/options/custom?airline=CJA&fltnum={0}&orig={1}&dest={2}" target="_blank" rel="noopener noreferrer">CJA{}便のフライト</a>
                    </div>"#, num, flight.dep, flight.arr));
            }
        }
        Err(_) => res.push_str(&format!(r#"<p>フライトが見つかりませんでした<br>臨時便を作成しますか？</p>
            <div class="result_a_frame">
                    <a href="https://dispatch.simbrief.com/options/custom?airline=CJA&orig={0}&dest={1}" target="_blank" rel="noopener noreferrer">CJA臨時便のフライト</a>
            </div>"#, icao_dep, icao_arr))
    };

    res
}
