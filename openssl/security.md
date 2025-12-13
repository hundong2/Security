### Phase 1. 개념 정립: 암호화의 3대장 (대칭키, 비대칭키, 해시)

가장 먼저 SHA256, AES128 같은 용어가 헷갈리는 이유는 \*\*"사용 목적"\*\*이 다르기 때문입니다. 이 셋은 SSL 통신 과정에서 각기 다른 역할을 맡습니다.

| 구분 | 알고리즘 예시 | 핵심 비유 | 역할 및 특징 |
| :--- | :--- | :--- | :--- |
| **대칭키 암호화** | **AES** (128, 256) | **금고와 열쇠** | • 암호화/복호화 키가 **동일**함.<br>• 속도가 매우 빠름.<br>• **실제 데이터(HTML, 이미지 등)를 전송할 때** 사용. |
| **비대칭키 암호화**<br>(공개키 암호화) | **RSA**, **ECC** | **자물쇠와 열쇠** | • 키가 두 개(공개키/개인키).<br>• 공개키로 잠그면(암호화), 개인키로만 열 수 있음(복호화).<br>• 속도가 느림.<br>• **AES 키를 안전하게 교환할 때**만 잠깐 사용. |
| **해시 함수** | **SHA** (256, 512) | **지문/밀랍인장** | • 데이터를 넣으면 고정된 길이의 문자열이 나옴.<br>• **복호화 불가능(단방향).**<br>• \*\*데이터가 변조되지 않았음을 증명(무결성)\*\*할 때 사용 (인증서 서명 확인 등). |

[Image of Symmetric vs Asymmetric Encryption]

> **핵심 요약:**
> SSL 핸드셰이크(연결) 과정에서는 \*\*비대칭키(RSA)\*\*를 사용해 서로 신원을 확인하고 \*\*대칭키(AES)\*\*를 안전하게 공유합니다. 그 후 실제 통신은 속도가 빠른 \*\*대칭키(AES)\*\*로 데이터를 주고받으며, 데이터가 위조되지 않았는지 \*\*해시(SHA)\*\*로 검증합니다.

-----

### Phase 2. 파일 확장자의 혼돈 풀기 (.pem, .crt, .key)

OpenSSL을 다룰 때 가장 혼란스러운 부분입니다. 결론부터 말하면 \*\*"내용물(Content)"\*\*과 \*\*"포맷(Encoding)"\*\*을 구분해야 합니다.

#### 1\. 포맷 (인코딩 방식)

  * **PEM (Privacy Enhanced Mail):**
      * 내용을 **Base64** 텍스트로 인코딩한 것.
      * `-----BEGIN CERTIFICATE-----` 로 시작합니다.
      * 리눅스(Apache, Nginx)에서 주로 사용. (텍스트 편집기로 열 수 있음)
  * **DER (Distinguished Encoding Rules):**
      * **바이너리(이진)** 포맷.
      * 텍스트 편집기로 열면 깨져 보임.
      * 주로 Java나 Windows 환경에서 사용.

#### 2\. 확장자 (관습적으로 붙이는 이름)

  * **.key:** 주로 \*\*개인키(Private Key)\*\*가 들어있습니다. (절대 유출 금지\!)
  * **.crt / .cer:** 주로 \*\*인증서(Certificate)\*\*가 들어있습니다. (공개키 + 서명 정보)
  * **.csr:** **인증서 서명 요청(Certificate Signing Request)** 파일. (인증기관에 보내는 신청서)
  * **.pem:** PEM 포맷으로 된 파일이면 인증서든 키든 다 붙일 수 있는 범용 확장자. (예: `cert.pem`, `key.pem`)

-----

### Phase 3. 인증서 체인 (Chain of Trust) 완벽 이해

인증서가 신뢰받는 원리는 \*\*"보증"\*\*의 연속입니다.

1.  **Root CA (최상위 인증기관):** 브라우저나 OS(Windows, Android 등)에 이미 내장되어 있는 "절대 존엄" 신뢰 기관.
2.  **Intermediate CA (중간 인증기관):** Root CA가 "얘는 믿을만해"라고 서명해준 중간 관리자. (Root CA는 너무 중요해서 오프라인에 숨겨두고, 실무는 얘네가 함)
3.  **Leaf Certificate (서버 인증서):** 우리가 구매해서 서버에 설치하는 인증서. Intermediate CA가 서명해줌.

**동작 순서:**

1.  클라이언트(브라우저)가 서버에 접속하면, 서버는 **[내 인증서 + 중간 인증서]** 뭉치를 보냅니다.
2.  브라우저는 **내 인증서**를 누가 서명했는지 봅니다. -\> "중간 인증서가 했네?"
3.  **중간 인증서**는 누가 서명했는지 봅니다. -\> "Root CA가 했네?"
4.  브라우저는 자신의 저장소에 있는 **Root CA** 목록을 확인합니다. -\> "어? 내가 아는 Root CA네? 통과\!"

-----

### Phase 4. OpenSSL로 실전 Deep Dive (순서대로 따라하기)

이제 터미널을 열고 직접 파일을 생성하며 익혀봅시다.

#### Step 1. 개인키(Private Key) 생성 (자물쇠의 열쇠 만들기)

가장 먼저 '나만의 비밀키'를 만듭니다.

```bash
# genrsa: RSA 알고리즘으로 생성
# aes256: 키 자체를 AES256으로 암호화 (비밀번호 설정)
# 2048: 키 길이 (비트 수, 길수록 안전)
openssl genrsa -aes256 -out private.key 2048
```

  * **결과:** `private.key` 파일 생성됨. (PEM 포맷)

#### Step 2. 인증서 서명 요청(CSR) 생성 (신청서 작성)

개인키를 사용해 "이 공개키를 포함해서 인증서를 만들어주세요"라는 신청서를 만듭니다.

```bash
# req: 요청 관리 명령어
# -new: 새 요청 생성
# -key: 사용할 개인키 지정
openssl req -new -key private.key -out request.csr
```

  * **결과:** `request.csr` 파일 생성됨. 이 안에 국가 코드, 조직명(CN) 등을 입력하게 됩니다.

#### Step 3. (실습용) 자체 서명 인증서(Self-Signed Certificate) 생성

원래는 CSR을 인증기관(Verisign, Sectigo 등)에 보내야 하지만, 실습을 위해 **내가 나를 보증하는** 인증서를 만듭니다. (Root CA 역할 흉내)

```bash
# x509: 인증서 표준 포맷 관리 명령어
# -days 365: 유효기간 1년
# -in: 입력할 신청서(CSR)
# -signkey: 서명할 키 (원래는 CA의 키여야 하지만, 여기선 내 키로 서명)
openssl x509 -req -days 365 -in request.csr -signkey private.key -out certificate.crt
```

  * **결과:** `certificate.crt` 생성됨. 이것이 서버에 설치할 **공개 인증서**입니다.

#### Step 4. 인증서 내용 뜯어보기 (검증)

사람이 읽을 수 있는 텍스트로 변환해 봅니다. 여기서 **알고리즘 정보**를 확인할 수 있습니다.

```bash
openssl x509 -in certificate.crt -text -noout
```

  * 출력 내용 중 `Signature Algorithm: sha256WithRSAEncryption` 같은 부분을 찾아보세요. (SHA256으로 해시를 뜨고 RSA로 서명했다는 뜻)

-----

### Phase 5. 주요 암호화 알고리즘 상세 (머리에 박제하기)

#### 1\. RSA (Rivest-Shamir-Adleman)

  * **원리:** 매우 큰 두 소수(Prime number)를 곱하는 것은 쉽지만, 그 곱한 값을 다시 소인수분해 하는 것은 슈퍼컴퓨터로도 몇백 년이 걸린다는 수학적 난제를 이용.
  * **특징:** 가장 널리 쓰이고 호환성이 좋음. 하지만 보안성을 높이려면 키 길이가 매우 길어져야 함(2048비트 이상).

#### 2\. ECC (Elliptic Curve Cryptography, 타원 곡선 암호)

  * **원리:** $y^2 = x^3 + ax + b$ 형태의 타원 곡선 위의 점을 더하고 빼는 복잡한 수학적 연산을 이용.
  * **특징:** RSA보다 **훨씬 짧은 키 길이**로 동일한 보안 강도를 가짐. (모바일 환경에서 성능이 좋음).
  * **용어:** 인증서에서 `ECDSA` (Elliptic Curve Digital Signature Algorithm)라고 보이면 이것입니다.

-----

### 📚 용어 정리 및 Reference

| 용어(약어) | 풀이 (Full Name) | 설명 |
| :--- | :--- | :--- |
| **SSL / TLS** | Secure Sockets Layer / Transport Layer Security | 보안 통신 규약. SSL은 구버전 명칭이나 관습적으로 사용하며, 실제로는 TLS 1.2, 1.3을 사용함. |
| **PKI** | Public Key Infrastructure | 공개키 기반 구조. 인증기관(CA), 인증서, 등록기관 등 보안 통신을 위한 총체적인 시스템. |
| **CA** | Certificate Authority | 인증 기관. 인증서를 발급하고 보증해주는 신뢰된 제3자 (예: DigiCert, Let's Encrypt). |
| **CSR** | Certificate Signing Request | 인증서 서명 요청. 내 공개키와 정보가 담긴 파일로, CA에 제출하여 인증서를 받기 위한 신청서. |
| **SHA** | Secure Hash Algorithm | 해시 함수. SHA-256은 256비트 길이의 해시값을 생성함. |
| **AES** | Advanced Encryption Standard | 고급 암호화 표준. 현재 가장 널리 쓰이는 대칭키 암호화 방식. |

**추천 학습 링크 (Hyperlinks):**

  * **[OpenSSL Command Cheatsheet](https://www.openssl.org/docs/manmaster/man1/)**: 공식 매뉴얼은 어렵지만 가장 정확합니다.
  * **[Mozilla SSL Configuration Generator](https://ssl-config.mozilla.org/)**: 웹서버(Nginx, Apache) 설정 시 가장 안전한 암호화 제품군(Cipher Suite) 설정을 만들어주는 도구입니다. 실무 필수 사이트입니다.
  * **[How HTTPS Works (Comic)](https://howhttps.works/)**: HTTPS 동작 원리를 만화로 매우 쉽게 설명한 사이트입니다. (강력 추천)

-----

### 💡 Next Step for You

이론을 보셨으니, 이제 손으로 익히는 것이 중요합니다. **"지금 바로 터미널을 켜서 위 OpenSSL 실습 명령어 3줄을 입력해보고, 생성된 `certificate.crt` 내용을 텍스트로 확인해 보시겠습니까?"** 이 과정을 한번만 거치면 파일 확장자에 대한 두려움이 사라질 것입니다.