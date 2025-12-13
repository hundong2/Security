# 인증서 Subject 필드(C, ST, L, O, OU, CN, emailAddress) 설명

CSR/인증서의 Subject에 들어가는 식별자들의 의미와 실제 작성 시 주의사항을 간단히 정리합니다.

## 예시 파일

[ISRG Root X1 PEM (Let's Encrypt 루트 인증서)](https://letsencrypt.org/certs/isrgrootx1.pem)

## 답변

Subject는 인증서 소유자를 식별하는 정보입니다. 각 필드의 의미는 다음과 같습니다.

- C (Country): 국가 코드. ISO 3166-1 alpha-2 두 글자 코드만 허용됩니다. 예: KR, US.
- ST (State or Province): 주/도/광역시 등 행정 구역. 예: Seoul, California. 빈칸/약칭 지양.
- L (Locality): 시/군/구 등 더 작은 지역. 예: Gangnam-gu.
- O (Organization): 조직(회사/기관) 공식 명칭. 법적 등록명 사용 권장.
- OU (Organizational Unit): 조직 내 부서/팀. 예: IT Department. 선택 사항.
- CN (Common Name): 일반적으로 서버 인증서에서는 도메인 이름(FQDN). 예: example.com 또는 *.example.com(와일드카드).
  - 중요: 현대 브라우저는 Subject Alternative Name(SAN)을 우선하며, SAN가 있으면 CN만 보고 신뢰하지 않습니다. CSR/인증서 생성 시 SAN에 정확한 도메인들을 넣는 것이 표준입니다.
- emailAddress: 주로 개인/클라이언트(S/MIME)에서 사용되는 이메일 주소. TLS 서버 인증서에서는 거의 사용하지 않으며, 이메일은 SAN에 넣는 것이 현대적입니다.

작성 팁과 주의사항:
- 값의 정확성: CA는 C/O 등 값을 실사하는 경우가 많습니다. 실제 법적/지리 정보 사용.
- CN vs SAN: 웹서버용은 반드시 SAN에 호스트명을 포함하세요. 예: DNS:example.com, DNS:www.example.com.
- 와일드카드: CN/SAN에 *.example.com을 쓰면 하위 1레벨에만 유효. 루트 도메인(example.com)에는 별도 항목 필요.
- 프라이버시: 개인용 인증서에는 불필요한 주소/부서 정보 최소화.
- 확인 명령어(맥 터미널):
  - CSR 확인:  
    openssl req -in request.csr -text -noout
  - 인증서 확인:  
    openssl x509 -in certificate.pem -text -noout

### 추가 자료

- [RFC 5280: X.509 인증서와 인증서 경로 검증](https://www.rfc-editor.org/rfc/rfc5280)
- [CA/Browser Forum Baseline Requirements](https://cabforum.org/baseline-requirements-documents/)
- [OpenSSL req(CSR) 매뉴얼](https://www.openssl.org/docs/manmaster/man1/req.html)