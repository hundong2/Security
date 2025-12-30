# MAC

- Message Authentication code
- text + key -> digest 

## HMAC 

- hash-based message authentication code 

- [Example HMAC Sender](./MAC/src/main.rs). 

```sh
cargo run                                                     ✔  23:01:12 
   Compiling MAC v0.1.0 (/Users/donghun2/workspace/Security/Real-worldCryptography/MAC)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `/Users/donghun2/workspace/Security/Real-worldCryptography/MAC/target/debug/MAC`
HMAC(tag): 42c9463e545c4a9d41caa1402d929b43e4f2b8d0a4738839d21ce2f7852e65b9
Is the message authentic? true
```

# Go에서 분기 없는 비교가 중요한 이유

비밀 데이터를 비교할 때 데이터 값에 따라 실행 경로(분기)가 달라지면 실행 시간 차이로 정보가 새어 나갈 수 있습니다. 분기 없는 루프는 모든 입력에 대해 동일한 작업 수·메모리 접근 패턴을 유지해 타이밍 공격을 막습니다.  

## 예시 파일

[crypto/subtle ConstantTimeCompare 문서](https://pkg.go.dev/crypto/subtle#ConstantTimeCompare)

## 답변

핵심 설명:
- 타이밍 공격은 “값에 따라 일찍 종료하거나 다른 경로로 실행되는” 분기에서 생깁니다. 예: 처음 다른 바이트를 발견하면 즉시 return 하는 비교는 입력 위치에 따라 시간이 달라집니다.
- 루프에서 v |= x[i] ^ y[i]는 모든 바이트를 끝까지 처리합니다. 각 반복에서 동일한 연산(로드→XOR→OR)을 수행하므로 데이터 값과 무관하게 실행 경로와 작업 수가 같습니다.
- 최종 판단도 분기 없이 처리해야 합니다. Go의 crypto/subtle에는 분기 없는 바이트/슬라이스 비교 함수가 준비되어 있습니다.

예시 코드(권장: 표준 라이브러리 사용):

````go
package main

import (
	"crypto/subtle"
	"fmt"
)

func constantTimeEqual(x, y []byte) bool {
	// 길이가 다르면 바로 0을 반환하는 분기는 “길이”가 비밀이 아닌 경우 허용됩니다.
	// 길이도 비밀이면 길이 차이를 v에 흡수하는 패턴을 사용하세요.
	if len(x) != len(y) {
		return false
	}
	// 분기 없이 모든 바이트를 처리
	var v byte
	for i := 0; i < len(x); i++ {
		v |= x[i] ^ y[i]
	}
	// 최종 판단도 분기 없는 연산(crypto/subtle) 사용
	return subtle.ConstantTimeByteEq(v, 0) == 1
}

func main() {
	a := []byte{1, 2, 3, 4}
	b := []byte{1, 2, 3, 4}
	c := []byte{1, 2, 9, 4}

	fmt.Println(constantTimeEqual(a, b))                     // true
	fmt.Println(constantTimeEqual(a, c))                     // false
	fmt.Println(subtle.ConstantTimeCompare(a, b) == 1)       // true
	fmt.Println(subtle.ConstantTimeCompare(a, c) == 1)       // false
}
````

실무 포인트:
- 비밀 비교에는 bytes.Equal 대신 crypto/subtle을 사용하세요. bytes.Equal는 빠르지만 입력에 따라 중간에 멈출 수 있습니다.
- 길이도 비밀이라면 길이 비교 분기를 제거하고, v |= byte(len(x)^len(y)) 같은 방식으로 차이를 흡수합니다.
- 분기뿐 아니라 “데이터 의존적 메모리 접근”도 피해야 합니다. 항상 같은 인덱스/범위를 순서대로 접근하세요.

### 추가 자료

- [crypto/subtle 패키지 문서](https://pkg.go.dev/crypto/subtle)
- [Timing attack(위키)](https://en.wikipedia.org/wiki/Timing_attack)
- [Cloudflare 블로그: 타이밍 공격 설명](https://blog.cloudflare.com/a-lesson-in-timing-attacks/)

# HMAC based key derivation funcction ( HKDF )

- [PRF-의사 난수 함수](./PRF.md). - pseudorandom function 
 	

# SipHash

- 짧은 인증 태그에 최적화 된 MAC
- [SipHash](./SipHash.md). 

## HMAC 

- 해시기반 맥 
- RFC2104, ANSI X9.71
- 입력 키(k)로부터 k1, k2가 파생됨.

### 순서

1. 먼저 기본키에서 두개의 키를 생성
- $$ k1 = k\oplus ipad $$
- $$ k2 = k\oplus opad $$ 
- ipad(내부 패딩), opad(외부패딩)은 상수

2. 키 k1을 메시지와 연결하고 해시 
3. 2의 결과를 키 k2에 연결하고 한번더 해시 
4. 최종 인증 태그를 생성 

- [길이 확장 공격(Length Extension Attack)](./LengthExtensionAttack.md). 