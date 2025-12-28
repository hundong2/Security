# echo 개행 포함 하지 않는 방법 

```sh
# 개행 포함(0a)
echo "abc" | hexdump -C
# 개행 없음
echo -n "abc" | hexdump -C
```