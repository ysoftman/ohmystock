# oh my stock

터미널 환경에서 관심 주식 가격 모티터링

## 주식 종목 데이터 다운로드

- 다운로드 <http://kind.krx.co.kr/corpgeneral/corpList.do?method=download>
- 다운로드하면 '상장범인목록.xls' 파일이지만 내용은 html 이다.
- euc-kr 인코딩이라 utf8 로 변경해서 저장한다.

## build

```bash
cargo build
```
