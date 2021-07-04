# SORACOMのGPSマルチユニットから取得した情報に、SORACOM Orbitで自宅からの距離情報を付与するコード類

## 概要
- やりたいこと: [SORACOMのGPSマルチユニット](https://soracom.jp/store/5235/)から得られる緯度経度の情報を、自宅からの距離(km)に変換し、SORACOM Harvestなど後段のサービスに送信する。変換処理に当たっては、[SORACOM Orbit](https://soracom.jp/services/orbit/)を利用する。
- 本リポジトリは、上記処理において、Orbitで使うWASMを生成するためのrustコード類である。
- コードは、[公式チュートリアル](https://users.soracom.io/ja-jp/docs/orbit/setup/)に掲載されているサンプルコードを改変して作成。


## 利用方法

1. GPSマルチユニットを買う。
1. [公式チュートリアル](https://soracom.jp/recipes_index/3830/)を参照し、GPSマルチユニットからHarvestにデータを定期送信し、Lagoonで可視化するよう一通り設定する。
1. 本リポジトリをローカルにclone。
1. `src/home_location.rs.sample`コピーして`src/home_location.rs`を作り、`lat_deg`と`lon_deg`にそれぞれ自宅の緯度と経度を入力する。
1. [公式チュートリアル](https://users.soracom.io/ja-jp/docs/orbit/setup/)を参照し、前提条件インストール、SAM、SORACOM CLIの設定を行い、本リポジトリをVSCodeで開く。その後、チュートリアルの手順にしたがってVSCode拡張機能のインストールなどを行う。
1. [公式チュートリアル](https://users.soracom.io/ja-jp/docs/orbit/deployment/?tab-1-2=selected)にしたがって、本リポジトリのルートで、`cargo build --release`としてWASMを生成、SORACOMのユーザーコンソールからWASMをアップロード。  
  なお、WASMモジュールのテストに当たっては、本リポジトリの`test-payload.json`を適宜使ってもよい。
1. [公式チュートリアル](https://users.soracom.io/ja-jp/docs/orbit/running/)を参考に、GPSマルチユニットのSIMグループでOrbitを利用するように設定。このとき、uplinkのみを使用するようにする。
1. 以上が正しく完了していれば、手順2で設定したSORACOM Lagoonのダッシュボードにおいて、パネルのQueryで、GPSマルチユニットから`distanceFromHome`というメトリクスが取得できるようになっている。  
  あとは、好きなように可視化したりアラートを設定したりする。