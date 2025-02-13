# 한빛미디어 이지 러스트 샘플 프로젝트 (BookShelf) 입니다.

이 깃헙은 한빛미디어 이지 러스트의  샘플 프로젝트인 "도서 관리 프로그램"의 완성된 코드를 담고 있습니다. 실습에 어려움을 겪으시면 이 저장소의 코드를 참조해주세요.

프로그램을 실제 동작시켜 보려면 PostgreSQL 서버가 필요합니다.

PostgreSQL 서버는 저장소를 복제하시고 프로젝트 최상위 디렉터리에서 다음  내용을 따라주세요.

## .env 파일 생성

먼저  .env 파일의 내용을 다음 형태로 미리 구성해주세요.

.env
```
DATABASE_URL=postgres://[username][:password]@[host][:port]/[database]
```

## Node.JS 환경 준비

두 번째로 데이터베이스 준비가 완료되면 node js 패키지를 설치합니다.

```
npm install
```

패키지 설치가 완료되면 세 번째로 도서 관리 프로그램을 실행해보면 됩니다.

## 프로그램 실행

```
cargo tauri dev
```

이 프로그램은 Rust에 tauri-cli가 설치되어 있어야 동작합니다. Tauri 프로그램 컴파일 중에 오류가 발생하면 rustup을 사용해 러스트 버전을 업데이트 해주세요.

## 문의는..

프로그램에 대한 문의는 깃헙 이슈로 요청 부탁드립니다.

감사합니다.
