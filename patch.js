export default (img, ext = undefined, quality = 80) =>
	nativeBinding.imgJxl(img, ext, quality);
