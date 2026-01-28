# oh my stock

터미널에서 관심 주식 가격 모니터링

## install and usage

```bash
# install
cargo uninstall ohmystock; cargo install ohmystock

# help
ohmystock -h

# 삼성전자 종목 현재 값 보기
ohmystock 삼성전자

# 삼성전자 회사 정보/주식참조URL 보기
ohmystock 삼성전자 -c

# 카카오 종목 1분마다 보기
ohmystock 카카오 -f

# 카카오 삼성전자 naver 종목 보기
ohmystock 카카오 삼성전자 naver -f
```
