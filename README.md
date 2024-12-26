# BARCODE PDF RENAMER

This application is designed to rename PDF documents to the value contained in the barcode found in the PDF 

# USAGE

First, build the application with cargo (was only tested on windows and linux), then define the absolute locations of the input and output folder inside the ```settings.json```  file. By default, the script will only look for the barcode in the first fifth of the document. The scan area can be changed but the application must be rebuilt. When running the application a system tray icon will appear, and clicking on it will give you the option to perform the scan.

NOTE: to successfully use this app you must first install poppler as described here https://pdf2image.readthedocs.io/en/latest/installation.html
