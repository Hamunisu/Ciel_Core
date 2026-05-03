# Ciel_Core

## 概要
***Ciel Japan Airlines***（バーチャルエアライン）のための便名検索エンジン  
  
**データ構造**: 空港名リスト（IcaoData）と便名データ（FlightsData）を[postcard](https://crates.io/crates/postcard)でバイナリ化  
  
**検索方法**: 事前にソートされたデータに対しbinary_search_by()を使用  
  
**出力内容**: SimBriefのurlにパラメータ（出発地、到着地、便名）を反映させたボタン

## 検索エンジンを使用したサイト
[https://hamunisu.github.io/Ciel_Airlines/](https://hamunisu.github.io/Ciel_Airlines/)

## 参考リンク
[SimBrief Dispatch Redirect Guide（パラメータについて）](https://forum.navigraph.com/t/dispatch-redirect-guide/5299)

## コードについて
本リポジトリはロジックの透明性のために公開されています  
シリアライズされたデータ（icao.ciel等）は同梱していませんのでビルドには別途postcard形式のデータが必要です  

## コピーライト
© 2026 Hamunisu / Ciel Japan Airlines
