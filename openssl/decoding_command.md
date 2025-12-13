# PEM/CSR 파일 내용 확인을 위한 OpenSSL 명령어

PEM, CSR 파일을 사람이 읽을 수 있는 텍스트로 확인하는 기본 명령어와 DER 포맷 처리 방법을 설명합니다.

## 예시 파일

[ISRG Root X1 PEM (Let's Encrypt 공식 루트 인증서)](https://letsencrypt.org/certs/isrgrootx1.pem)

## 답변

다음 명령어는 macOS 터미널에서 그대로 실행할 수 있습니다. 파일 포맷(PEM/DER)과 종류(인증서/CSR/키)에 따라 적절히 선택하세요.

- 인증서 내용 보기 (PEM/DER 자동 처리)
  - PEM/CRT/CER 파일:  
    openssl x509 -in certificate.pem -text -noout
  - DER 바이너리 파일:  
    openssl x509 -in certificate.der -inform DER -text -noout

- CSR 내용 보기
  - PEM CSR:  
    openssl req -in request.csr -text -noout
  - DER CSR:  
    openssl req -in request.der -inform DER -text -noout

- 개인키(PEM) 확인 및 무결성 점검
  - 암호화된 키도 비밀번호를 물어보며 읽음:  
    openssl pkey -in private.key -text -noout

- 공개키 추출/확인
  - 인증서에서 공개키 보기:  
    openssl x509 -in certificate.pem -noout -pubkey
  - 개인키에서 공개키 보기:  
    openssl pkey -in private.key -pubout

- 파일이 무엇인지 자동 판별(빠른 점검)
  - 헤더만 요약해 보기:  
    openssl x509 -in certificate.pem -noout -subject -issuer -dates

예시 실행:
- 루트 인증서 텍스트 보기:  
  curl -o isrgrootx1.pem https://letsencrypt.org/certs/isrgrootx1.pem  
  openssl x509 -in isrgrootx1.pem -text -noout

### 추가 자료

- [OpenSSL x509 매뉴얼](https://www.openssl.org/docs/manmaster/man1/x509.html)
- [OpenSSL req(CSR) 매뉴얼](https://www.openssl.org/docs/manmaster/man1/req.html)