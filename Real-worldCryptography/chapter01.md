# 대칭 암호학

- Symmetric encryption algorithm ( cipher )
- primitive(원시적인) : 암호화 알고리즘의 한 종류로 사용, 암호학에서 가장 작고 유용한 구성요소, 프로토콜을 만들기 위해 다른 프리미티브와 함께 쓰이기도 한다. 

- $대칭 암호화 \subset 대칭암호학, 비밀키 암호학$
- 대칭 고급얌효화 표준 : `Advanced Encryption Standard` ( AES ) - NIST 

# 비대칭 암호학 

- 비대칭 암호학 프리미티브는 `key exchange(키교환)` 이다.  
- 최초로 발표된 공개키 알고리즘은 (Diff디피 + 헬먼Hellman)이름을 따서 명명된 DH키 교환 알고리즘이다. 

# RSA 알고리즘

- 로널드 리베스트, 아디 샤미르, 레너드 애들먼의 성늘따서 명명
- 공개키 알고리즘(또는 비대칭 암호화)와 (디지털)서명 체계라는 두가지 프리미티브를 포함한다.  $공개키알고리즘, 디지털 서명 \subset 비대칭 암호학$
- 두가지 비밀키, 공개키로 작동 
- 비대칭적 관점을 제공, 즉, 누구나 공개 키로 암호화 할 수 있지만 비밀키의 소유자만 메시지를 해독할 수 있다. 
- 누구나 공개키를 통해 암호화 할 수 있지만, 암호화된 데이터는 비밀키가 있어야 복호화가 가능.  

# 디지털 서명 : 펜과종이의 서명과 그리 다르지 않다. 

- RSA는 비대칭 암호화 알고리즘과 더불어 디지털 서명(digital signature)알고리즘도 제공.  
- 암호화 서명은 암호화 인증서를 제공.
- 암호화 서명은 `위조가 불가능`

# SHA256

- SHA256으로 다운로드 -> 해시 ( = 다이제스트 (digest)) 
- SHA256 hash 함수를 이용하여 입력을 `고유식별자`로 변환한다. 
- `고유식별자`를 통해 `무결성`과 `진본성`을 얻을 수 있다.  
- `제2 역상 저항성(second preimage resistance)`라는 해시 함수의 보안 속성 
- 256bit(32byte)로 출력은 항상 동일한 자리수 
- 16진수로 된 64개의 알파벳 및 숫자로 인코딩 된다. 
- 주어진 출력을 가지고 입력을 찾을 수 없다. ( 해시 함수의 이러한 속성을 단방향(one-way))이라고 한다. 

```sh
openssl dgst -sha256 decoding_command.md 
SHA2-256(decoding_command.md)= 5b8a036f0e32d750872cf7f0e9993349c923ffe0622c152399c6976c07aaf03f
```

## 해시 함수의 보안 제약

- 역상 저항성
- 제2 역상 저항성
- 충돌 저항성 

# birthday paradox

- [birthday paradox](./birthdayparadox.md). 

## tuple hash

- 제2 역상 저항성 문제 있는 예제 

```sh
echo -n "Alice""Bob""100""15" | openssl dgst -sha3-256      0|1 ✘  21:34:43 
SHA3-256(stdin)= 34d6b397c7f2e8a303fc8e39d283771c0397dad74cef08376e27483efc29bb02
```

```sh
echo -n "Alice""Bob""1001""5" | openssl dgst -sha3-256          ✔  21:34:56 
SHA3-256(stdin)= 34d6b397c7f2e8a303fc8e39d283771c0397dad74cef08376e27483efc29bb02
```