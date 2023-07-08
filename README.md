# WOMBDER
is a simple lightweight CLI based steganography tools that written in Rust. it perform embedding a text message to a .jpg or png image format. its useful to send a secret message without anyone knows it.
## INSTALLATION
just clone this repository with git command in your linux or MAC
```
git clone https://github.com/rakutatusan/wombder.git
```
and then run 
```
chmod +x install.sh
```
to make install script can run and executed
then, it'll installed to your system.
## USAGE
```
wombder -h
```
it will show you the help section to show you how to use the app. and this is the command for it:

``
Embedding a text message to an image:
``
```
-sm <text_file> -sr <image_file> 
```
-sm is stands for stego message. which is the text file with your secret message on it.
and -sr stands for stego result which is the image file where u want to put the secret message on.
then, it will prompt you to enter a password to stego embedded file. so other who want extracting stego secret message is need to know the password.

``
extracting a text message from an image:``


once you know the stego file, you can extract the secret message by typing:
```
-sf <image_file> -esf <output_file>
```
where -sf is stand for stego file. which is the stego included file and it is the picture.
and -esf is stand for extracted stego file. which is the outpout of message in stego included file. you can replace `` <output_file>`` by name of message outpout that you want. such as message.txt

