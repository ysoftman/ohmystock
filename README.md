# oh my stock

터미널에서 관심 주식 가격 모니터링

## 주식 종목 데이터

- 다운로드 <http://kind.krx.co.kr/corpgeneral/corpList.do?method=download>
- 다운로드하면 '상장범인목록.xls' 파일이지만 내용은 html 이다.
- euc-kr 인코딩이라 utf8 로 변경해서 저장한다.

## build and deploy

```bash
# 테스트(-f, --follow flag)
cargo run -- -f 카카오

# 빌드 및 실행
cargo build
target/debug/ohmystock 카카오

# cargo ohmystock 로그인
# https://crates.io/me 에서 토큰 생성(토큰이 만료된 경우 다시 생성)
cargo login cio75TLro7cxCGOwjvNmFu6NSBAnLR8troQ

# cargo.toml 버전업 수정 -> git 커밋 -> cargo 로 배포
# --allow-dirty : git 커밋 없이 로컬 변경 사항이 있는채로 배포 허용
cargo publish
```

## [install and usage](README_USAGE.md)
