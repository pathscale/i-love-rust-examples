#!/bin/sh

# Generate self signed root CA cert
openssl req -nodes -x509 -newkey rsa:2048 -keyout ca.key -out ca.crt -subj "/C=HK/ST=KLN/L=Kowloon/O=CV/OU=root/CN=`hostname -f`/emailAddress=admin@cv.com"


# Generate server cert to be signed
openssl req -nodes -newkey rsa:2048 -keyout server.key -out server.csr -subj "/C=HK/ST=KLN/L=Kowloon/O=CV/OU=r/CN=`hostname -f`/emailAddress=admin@cv.com"

# Sign the server cert
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt

# Create server PEM file
cat server.key server.crt > server.pem


# Generate client cert to be signed
openssl req -nodes -newkey rsa:2048 -keyout client.key -out client.csr -subj "/C=HK/ST=KLN/L=Kowloon/O=CV/OU=rr/CN=`hostname -f`/emailAddress=@cv.com"

# Sign the client cert
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAserial ca.srl -out client.crt

# Create client PEM file
cat client.key client.crt > client.pem

