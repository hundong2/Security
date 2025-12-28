# 해시 함수의 역상·제2 역상·충돌 저항성 상세 설명
세 가지 핵심 보안 속성을 직관과 수학으로 정리하고 코드·CLI 실습으로 확인합니다.

## 예시 파일
[RFC 6234: SHA 알고리즘(텍스트)](https://www.rfc-editor.org/rfc/rfc6234.txt)

## 답변
핵심 개념(비유와 함께):
- 역상 저항성(Preimage Resistance): 출력(지문)만 보고 원래 입력(사람)을 찾기 매우 어렵다는 성질입니다. 수학적으로, 해시 출력 길이를 `b`라 하면 평균 작업량은 대략 $2^b$입니다.
- 제2 역상 저항성(Second Preimage Resistance): 어떤 특정 입력 A가 주어졌을 때, A와 다른 입력 B를 찾아 해시가 같게 만드는 것이 매우 어렵다는 성질입니다. 난이도는 역상 저항성과 비슷하게 $2^b$ 수준으로 봅니다.
- 충돌 저항성(Collision Resistance): 아무 두 입력(특정 A에 묶이지 않음) 사이에서 같은 해시를 갖는 한 쌍을 찾기 어려운 성질입니다. 생일 역설 때문에 기대 작업량은 $2^{b/2}$ 근처입니다. 대략 충돌 확률은 샘플 수 `n`에 대해
  $$
  p \approx 1 - e^{-\frac{n(n-1)}{2\cdot 2^b}}
  $$
  이며 $p \approx 0.5$가 되는 `n`은 대략 $1.177 \cdot 2^{b/2}$입니다.

직관적 비유:
- 역상: 금고의 시리얼(해시)만 보고 원래 열쇠(입력)를 역으로 만드는 건 불가능에 가까움.
- 제2 역상: 특정 열쇠 A와 동일하게 동작하는 다른 열쇠 B를 만들기는 매우 어려움.
- 충돌: 아무 열쇠 두 개가 우연히 같은 금고를 여는(같은 해시를 내는) 상황을 찾는 일이지만, 우연치고는 아주 오래 걸림(생일 역설로 근본 난도가 반으로 줄어듦).

보안 크기 감각:
- SHA-256(`b=256`)은 충돌 탐색이 $2^{128}$ 수준으로 현재 실용 범위를 넘습니다.
- SHA-1(`b=160`)은 2017년 SHAttered, 2020년 선택-접두 충돌로 실사용 금지(충돌 저항성 붕괴).
- SHA-3(Keccak) 계열은 구조가 달라 안전성 평가가 독립적이며, cSHAKE/KMAC 등 도메인 분리·튜플 인코딩을 내장한 변형이 표준화됨.

최신 트렌드 요약(2025):
- SHA-1 퇴출 완료(코드 서명, TLS 등). Git도 SHA-256 지원로 마이그레이션.
- SHA-2(SHA-256/384/512)는 여전히 표준 실무 기본. 포스트양자 맥락에서도 해시에는 주로 Grover로 보안비트가 절반가량으로 평가되지만, 출력 길이를 넉넉히 잡으면 대응 가능.
- SHA-3는 선택적으로 채택(하드웨어·도메인 분리 필요 시). 고성능 해시로는 BLAKE3가 실무에서 인기(비표준이지만 빠름); 충돌/역상 모델은 여전히 전통적 해시 안전성 가정을 따름.
- 문서·아티팩트 무결성 검증은 SHA-256이 디폴트, 장기 보존이나 법적 무결성에는 SHA-512/멀티해시 병행도 채택.

코드 예제

```python
# Python: 역상/제2역상/충돌 직관 확인(요약)
import hashlib, os

def h256(b: bytes) -> bytes:
    return hashlib.sha256(b).digest()

# 1) 바이트 1개만 달라도 해시가 완전 달라짐(민감도 확인)
a = b"AliceBob10015"
b = b"AliceBob1001" + b"5"  # 같은 연결 문자열이 되면 해시가 '같음' -> 경계 인코딩 필요
print("A==B bytes?", a == b)
print("sha256(A)=", hashlib.sha256(a).hexdigest())
print("sha256(B)=", hashlib.sha256(b).hexdigest())

# 2) 충돌 데모: 안전한 SHA-256은 실제 충돌을 못 찾으므로,
#    16비트로 '잘라낸 해시'에서 임의 충돌을 빠르게 관찰(생일 역설 체감용)
def short16(b: bytes) -> int:
    return int.from_bytes(h256(b)[:2], "big")  # 상위 16비트
seen = {}
for i in range(1, 20000):
    x = os.urandom(32)
    s = short16(x)
    if s in seen:
        print("Truncated collision found at 16 bits (easy):", i)
        break
    seen[s] = x
```

```cpp
// C++ (OpenSSL): SHA-256 해시 계산
// 빌드: clang++ sha_demo.cpp -lcrypto -o sha_demo
#include <openssl/sha.h>
#include <iostream>
#include <vector>

std::string hex(const unsigned char* d, size_t n){
    static const char* k="0123456789abcdef";
    std::string s; s.reserve(n*2);
    for(size_t i=0;i<n;i++){ s.push_back(k[d[i]>>4]); s.push_back(k[d[i]&0xF]); }
    return s;
}

int main(){
    const char* msg = "AliceBob10015";
    unsigned char out[SHA256_DIGEST_LENGTH];
    SHA256(reinterpret_cast<const unsigned char*>(msg), strlen(msg), out);
    std::cout << "SHA-256: " << hex(out, sizeof(out)) << "\n";
    return 0;
}
```

```csharp
// C#: SHA-256 해시 계산
// dotnet-script 또는 콘솔앱에서 사용
using System;
using System.Security.Cryptography;
using System.Text;

class Program {
  static void Main() {
    var data = Encoding.UTF8.GetBytes("AliceBob10015");
    using var sha = SHA256.Create();
    var hash = sha.ComputeHash(data);
    Console.WriteLine(BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant());
  }
}
```

OpenSSL 실습(zsh, macOS)

```sh
# 1) 정확한 바이트로 해시(개행 없음)
printf "%s" "AliceBob10015" | openssl dgst -sha256

# 2) 경계 모호성 시연: 단순 연결은 위험
printf "%s%s%s%s" "Alice" "Bob" "100" "15" | openssl dgst -sha3-256
printf "%s%s%s%s" "Alice" "Bob" "1001" "5" | openssl dgst -sha3-256  # 같은 문자열이면 같은 해시

# 3) 튜플 안전 직렬화(길이 접두, 간단 netstring)
printf "5:Alice,3:Bob,3:100,2:15," | openssl dgst -sha3-256
printf "5:Alice,3:Bob,4:1001,1:5," | openssl dgst -sha3-256  # 서로 다른 해시

# 4) 충돌 직관(16비트로 '잘라' 비교해 충돌을 빠르게 체감)
for i in {1..5000}; do
  head -c 32 /dev/urandom | openssl dgst -sha256 | awk '{print substr($2,1,4)}'
done | sort | uniq -d | head
# 실해시(SHA-256 전체) 충돌은 실용 시간 내 발견 불가.
```

실무 베스트 프랙티스:
- 해시를 데이터 구조에 적용할 때는 “경계가 보장된 인코딩”(길이 접두, CBOR/ASN.1, NIST 800-185의 encode_string 등)을 사용합니다.
- 무결성 검증에는 SHA-256 이상을 기본으로, 장기보존이나 법적 용도에는 SHA-512 또는 다중 해시 병행을 고려합니다.
- SHA-1은 사용하지 않습니다(충돌 저항성 붕괴).
- 포스트양자 고려 시 출력 길이를 충분히 확보(예: SHA-256/512 유지)하고, 서명/키교환은 PQC로 전환합니다.

### 추가 자료
- [RFC 6234: US Secure Hash Algorithms (SHA and HMAC)](https://www.rfc-editor.org/rfc/rfc6234.txt)
- [NIST FIPS 180-4: SHA-2 표준](https://csrc.nist.gov/publications/detail/fips/180/4/final)
- [NIST FIPS 202: SHA-3 표준](https://csrc.nist.gov/publications/detail/fips/202/final)
- [NIST SP 800-185: cSHAKE/KMAC/TupleHash](https://csrc.nist.gov/publications/detail/sp/800-185/final)
- [SHAttered(SHA-1 충돌)](https://shattered.io/)
- [Chosen-Prefix Collision on SHA-1 (2020)](https://sha-mbles.github.io/)

### 용어사전
- 역상 저항성: 출력만 주어졌을 때 원래 입력을 찾기 어려운 성질.
- 제2 역상 저항성: 특정 입력과 같은 해시를 내는 다른 입력을 찾기 어려운 성질.
- 충돌 저항성: 아무 두 입력 사이에 같은 해시를 갖는 쌍을 찾기 어려운 성질.
- 생일 역설: 무작위 표본 수가 약 $1.177\cdot 2^{b/2}$일 때 절반 확률로 충돌이 나타나는 현상.
- 도메인 분리: 해시 목적을 구분하는 태그를 넣어 함수 오용/교란을 방지하는 기법.
- 튜플 인코딩: 각 요소를 길이와 함께 직렬화해 경계 모호성으로 인한 충돌 위험을 제거하는 방법.
- SHA-2/3: NIST 표준 해시 계열로 현재 실무의 기본(SHA-2)과 대안(SHA-3).
- BLAKE3: 비표준 고성능 해시. 무결성·동일성 검사에 쓰이나 표준 요구 환경에선 NIST 계열 권장.
