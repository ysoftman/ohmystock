# oh my stock

터미널에서 관심 주식 가격 모니터링

## install and usage

```bash
# install
cargo uninstall ohmystock; cargo install ohmystock

# help
ohmystock -h

# 전체 종목 목록 보기
ohmystock -l

# 키워드로 종목 검색 (종목명, 종목코드, 업종)
ohmystock -l 전자

# 삼성전자 종목 현재 값 보기
ohmystock 삼성전자

# 삼성전자 회사 정보/주식참조URL 보기
ohmystock 삼성전자 -c

# 카카오 종목 1분마다 보기
ohmystock 카카오 -f

# 카카오 삼성전자 naver 종목 보기
ohmystock 카카오 삼성전자 naver -f
```

## build and deploy

```bash
# 테스트(-f, --follow flag)
cargo run -- -f 카카오

# 빌드 및 실행
cargo build
target/debug/ohmystock 카카오

# cargo 로그인
# https://crates.io/me 에서 토큰 생성함
# 로그인 하면 ~/.cargo/credentials.toml 에 토큰 저장됨
cargo login

# cargo.toml 버전업 수정 -> git 커밋 -> cargo 로 배포
# --allow-dirty : git 커밋 없이 로컬 변경 사항이 있는채로 배포 허용
cargo publish
```

## 주식 종목 데이터 업데이트시

```bash
# KRX에서 상장법인목록 다운로드 후 UTF-8 변환하여 data/stock_list.html 에 저장
./update_stock_list.sh
```
