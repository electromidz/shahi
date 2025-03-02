# SHAHI
# Layer one blockchain

🛠 Step 1: Generate a Self-Signed CA Certificate

Run the following command to generate a CA key and CA certificate:

openssl req -x509 -newkey rsa:4096 -keyout ca.key -out ca.crt -days 365 -nodes -subj "/CN=MyCA"

    ca.key → Private key for the CA (keep it safe).
    ca.crt → Public certificate for the CA.

🛠 Step 2: Generate a Server Key and CSR

Now, generate the server's private key and Certificate Signing Request (CSR):

openssl req -newkey rsa:2048 -keyout server.key -out server.csr -nodes -subj "/CN=localhost"

    server.key → Private key for the server.
    server.csr → CSR to request a certificate from the CA.

🛠 Step 3: Sign the Server Certificate with the CA

Create a config file (server.ext) with proper extensions:

echo "authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
" > server.ext

Then sign the server certificate using the CA:

openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 365 -extfile server.ext

    server.crt → Properly signed server certificate.

🛠 Step 4: Use ca.crt on the Client

The server uses server.crt and server.key, but the client should trust ca.crt, not server.crt.
