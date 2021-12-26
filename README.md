# Saltcrypt

Saltcrypt is a simple in-place file encryption tool using HKDF and AEAD.


<img src="https://github.com/kolemikko/saltcrypt/blob/0d2dc8799eb1db1c14f9d40080d42a7f134acbde/doc/screenshot.png" width=45% height=45%>


Saltcrypt uses Key Derivation Function with HMAC-256 (HKDF) to create strong hashes out of provided password and salt, but also name of the target file. 
Even when using same gredentials, created hashes are always unique for every file, which allows small safety measure: renamed encrypted files can't be opened anymore even with correct gredentials. Unless renamed back to original name.
ChaCha20 and Poly1305 based Authenticated Encryption with Associated Data (AEAD) cipher is used for the file encryption.

Saltcrypt can be used as a command line tool by providing needed arguments, if no arguments are provided, graphical user interface will be started.
