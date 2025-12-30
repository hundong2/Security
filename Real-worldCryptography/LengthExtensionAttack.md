# 길이 확장 공격(Length Extension Attack)

Merkle–Damgård 계열 해시(MD5, SHA-1, SHA-256 등)에서, H(m)과 m의 길이를 알면 H(m || pad(m) || x)를 새로 계산할 수 있는 성질을 악용하는 공격입니다. 특히 MAC을 Hash(key || message)로 만들면 태그를 위조할 수 있습니다.

## 예시 파일

[Length extension attack (Wikipedia)](https://en.wikipedia.org/wiki/Length_extension_attack)

## 답변

핵심 아이디어:

- Merkle–Damgård 해시는 내부 상태를 마지막 해시값으로 노출합니다. 공격자는 이 상태를 시작점으로 이어서 블록을 더 처리해도 “정상 해시”와 동일한 결과가 나옵니다.
- 따라서 MAC = Hash(key || msg)일 때, 공격자는 태그와 msg만 알고도 msg에 x를 덧붙인 메시지에 대한 올바른 태그를 만들 수 있습니다(키를 모른 채로).

취약 시나리오(예):

- 서버가 서명: tag = SHA-256(key || "action=read&user=alice")
- 공격자는 tag와 원문을 획득.
- 길이 확장으로 "&admin=true"를 덧붙인 새 메시지와 새 태그를 생성해 검증을 통과.

데모(macOS, hashpump 사용):

```sh
# 설치
brew install hashpump

# 가정: 서버가 알려준 원래 태그(16진)와 원문, 키 길이를 추정(예: 16바이트)
ORIG_TAG="d2b...abcd"                      # 예시(실제 태그로 대체)
ORIG_MSG="action=read&user=alice"
APPEND="&admin=true"
KEYLEN=16

# 길이 확장 수행 → 새 태그와 패딩 포함 새 메시지 출력
hashpump -s "$ORIG_TAG" -d "$ORIG_MSG" -a "$APPEND" -k $KEYLEN
# 출력:
# New hash: <NEW_TAG>
# New string: <ORIG_MSG><padding><APPEND>
```

검증(개념적 파이썬 확인):

```python
import hashlib

key = b"x"*16  # 서버 비밀(공격자는 모름)
orig = b"action=read&user=alice"
append = b"&admin=true"

# 서버가 가진 '진짜' 원래 태그
orig_tag = hashlib.sha256(key + orig).hexdigest()

# 공격자가 hashpump로 만든 new_msg, new_tag를 서버에 제출하면,
# 서버는 hashlib.sha256(key + new_msg).hexdigest() == new_tag 로 통과(위조 성공).
```

왜 HMAC은 안전한가?

- HMAC은 내부/외부 키 패딩(ipad/opad)로 두 겹 해시를 하며, 마지막 해시값만으로는 내부 상태를 재현할 수 없습니다. 설계상 길이 확장 공격에 면역입니다.

대응 방법:

- 절대 MAC = Hash(key || msg) 같은 “순진한” 구성 사용 금지.
- HMAC-SHA-256/512, CMAC, 또는 AEAD(예: AES-GCM, ChaCha20-Poly1305) 사용.
- SHA-3(스펀지)는 Merkle–Damgård가 아니어서 동일한 길이 확장 취약성이 없습니다. 그래도 MAC은 HMAC/KMAC 등 표준을 사용하세요.

포인트 정리:

- 길이 확장은 “해시의 구조적 성질” 문제이며, 해시 자체를 바꾸지 않고도 태그 위조가 가능.
- 키 길이를 모르면 여러 길이를 시도해볼 수 있어 실무에서 특히 위험.
- 해결책은 안전한 MAC(특히 HMAC) 채택과 안전한 직렬화(경계 보장)입니다.

### 추가 자료

- [RFC 2104: HMAC](https://www.rfc-editor.org/rfc/rfc2104)
- [FIPS 180-4: SHA-2 표준](https://csrc.nist.gov/publications/detail/fips/180/4/final)
- [HashPump (GitHub)](https://github.com/bwall/HashPump)