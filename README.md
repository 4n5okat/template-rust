# Rust(actix-web)のサンプルリポジトリ

## 概要

- Rustとフレームワークのactix-webを利用したバックエンドアプリケーションサンプルリポジトリ

## 目次

- [Rust(actix-web)のサンプルリポジトリ](#rustactix-webのサンプルリポジトリ)
  - [概要](#概要)
  - [目次](#目次)
  - [共通事項](#共通事項)
  - [事前準備](#事前準備)
  - [開発環境の詳細](#開発環境の詳細)
    - [ディレクトリ構成](#ディレクトリ構成)
    - [srcディレクトリの構成詳細](#srcディレクトリの構成詳細)
      - [各ディレクトリの責任範囲](#各ディレクトリの責任範囲)
  - [開発の進め方](#開発の進め方)
    - [Dev container](#dev-container)
    - [docker compose](#docker-compose)
  - [基本コマンド一覧](#基本コマンド一覧)
  - [参考リンク](#参考リンク)

## 共通事項

- [採用Gitの運用ルール](/docs/README.git.md)

## 事前準備

- [開発環境構築](/docs/README.environment-building.md)

## 開発環境の詳細

### ディレクトリ構成

~~~sh
.
├── .devcontainer/
│   └── devcontainer.json
├── .vscode(VSCodeの設定)/
│   └── settings.json
├── docs(ドキュメント群)/
│   ├── README.environment-building.md
│   ├── README.git.md
│   ├── README.make.md
│   ├── README.scoop-package.md
│   └── README.scoop.md
├── infrastructure /
│   └── rust/
│       └── Dockerfile
├── src/
│   ├── application/
│   │   └── use_cases
│   ├── domain/
│   │   └── entities
│   ├── infrastructure/
│   │   ├── db
│   │   ├── devices
│   │   ├── external_interfaces
│   │   └── web
│   ├── interface/
│   │   ├── controllers
│   │   ├── gateways
│   │   └── presenters
│   └── main.rs
├── .dockerignore
├── .env.example
├── .gitignore
├── API.rest(REST Client)
├── Cargo.toml
├── compose.build.yml(本番イメージ作成してローカルで検証用)
├── compose.yml
├── Makefile
└── README.md
~~~

[ディレクトリツリーリンク](https://tree.nathanfriend.com/?s=(%27optiGs!(%27fancyB~fullPath!falO~Wail8gSlashB~rootDotB)~J(%27J%27.6*6U.vscode%7BVSCode%E3%81%AE%E8%A8%AD%E5%AE%9AXOtt8gsUdocs%7B%E3%83%89%E3%82%AD%E3%83%A5%E3%83%A1%E3%83%B3%E3%83%88%E7%BE%A4X3envirGmH-build8g5git5makN-packagN.md24%20*rust70Dock9fiQsrc*applicatiG7uO_caOs*doma87Hities*47db7deviceYext9nal_LYweb*L7cGWoll9YgatewayYpresH9s*ma8.rs2.dock9K.env.exampQ.gitKAPI.rest%7BREST%20CliH%7D2Cargo.tomlFbuild.yml%7B%E6%9C%AC%E7%95%AA%E3%82%A4%E3%83%A1%E3%83%BC%E3%82%B8%E4%BD%9C%E6%88%90%E3%81%97%E3%81%A6%E3%83%AD%E3%83%BC%E3%82%AB%E3%83%AB%E3%81%A7%E6%A4%9C%E8%A8%BC%E7%94%A8%7DFyml2MakefiQ3md%27)~v9siG!%271%27)*200%20%202%5Cn3README.48frasWucture5.md*36devcGta897*08in9erB!WueF2compoO.GonHentJsource!Kignore2L8t9faceNe5scoopOseQle2U.jsG2WtrX%7D*Ys7%01YXWUQONLKJHGFB987654320*)

### srcディレクトリの構成詳細

#### 各ディレクトリの責任範囲

各ディレクトリやファイルの主な役割は以下の通りです。

| パス | レイヤー (クリーンアーキテクチャ) | 責任範囲 |
| ---- | ---- | ---- |
| src/domain/entities | Entities | ドメインエンティティ。アプリケーションの核となるビジネスルールとロジックを定義します。他のどのレイヤーにも依存しない、最も内側の層です。 |
| src/application/use_cases | Use Cases | ユースケース。アプリケーション固有のビジネスロジックを実装します。エンティティを操作し、特定の目的（例：ユーザー登録）を達成します。 |
| src/interface/ | nterface Adapters | インターフェースアダプター（抽象）。ユースケース層とインフラ層の間の変換を担当するインターフェース（Rustのトレイト）を定義します。 |
| src/interface/controllers | - | コントローラー。HTTPリクエストなどの外部からの入力を受け取り、それをユースケースが理解できる形式に変換して、対応するユースケースを呼び出します。 |
| src/interface/gateways | - | ゲートウェイ。データベースや外部APIなど、外部システムへのアクセス方法を定義するインターフェース（トレイト）です。リポジトリパターンなどがここに属します。 |
| src/interface/presenters | - | プレゼンター。ユースケースからの出力データを、HTTPレスポンスなどの外部システムに適した形式に変換します。 |
| src/infrastructure/ | Frameworks & Drivers | インフラストラクチャ（具象）。interfaceで定義されたトレイトの具体的な実装を行います。フレームワークやドライバなど、外部の技術的な詳細をここに閉じ込めます。 |
| src/infrastructure/db | - | interface/gatewaysで定義されたDBアクセス用トレイトの具体的な実装（例：dieselやsqlxを使ったコード）。 |
| src/infrastructure/devices | - | もしあれば）特定のハードウェアデバイスとの通信など、具体的な実装。 |
| src/infrastructure/external_interfaces | - | 外部API（REST、gRPCなど）と通信するためのinterface/gatewaysの具体的な実装。 |
| src/infrastructure/web | - | actix-webのルーティング設定、サーバー起動、ミドルウェアなど、Webフレームワークに依存する具体的な実装。interface/controllersが利用する具体的な処理を記述します。 |
| src/main.rs | アプリケーションの起点） | エントリーポイント。各レイヤーの依存関係を構築（DI: 依存性の注入）し、アプリケーション（actix-webサーバー）を起動します。 |

## 開発の進め方

### Dev container

VS Codeなどでdevcontainerを利用できるようにプラグインをインストールする
※ 既にVS Codeの環境が整っている場合は下記コマンドでプラグインをインストールできます。

~~~sh
code --install-extension ms-vscode-remote.remote-containers
~~~

### docker compose

[基本コマンド](#基本コマンド一覧)を参照のこと

## 基本コマンド一覧

- [Makeコマンド一覧](/docs/README.make.md)

多用コマンド

~~~sh
# git clone後に初期起動するときのコマンド
make first-up-build

# make first-up-buildを実行した後で再度コンテナ起動したい場合のコマンド
make up

# コンテナログを確認するときのコマンド
make logs-f

# compose.ymlに記載しているサービスのコンテナの停止・削除したい場合のコマンド
make down

# compose.ymlに記載している不要なイメージ、ボリュームなどを削除したい場合のコマンド
make down-rmi

# 全体的な不要なリソースの完全削除するコマンド
make down-all
~~~

~~~sh
# Rustコンテナにログイン
make login
~~~

## 参考リンク

- [Rust公式](https://www.rust-lang.org/ja)
- [Cargo.toml チートシート](https://blog1.mammb.com/entry/2024/01/22/090000)
