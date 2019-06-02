import init, { do_fast_blur } from '../../pkg'

const IMAGE_SRC = '/dist/static/test.jpg'
const VIDEO_SRC = '/dist/static/bigbuck.webm'
const MAX_RADIUS = 100

const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')

const radiusSlider = document.getElementById('radiusSlider')

const imageEle = document.getElementById('image')
const videoEle = document.getElementById('video')

imageEle.src = IMAGE_SRC
videoEle.src = VIDEO_SRC

function start() {
  bindSliderEvent()

  processBlur(5)
}

function processBlur(radius, ele=imageEle) {
  const width = ele.videoWidth || ele.naturalWidth
  const height = ele.videoHeight || ele.naturalHeight
  canvas.width = width
  canvas.height = height
  ctx.drawImage(ele, 0, 0)

  const imageData = ctx.getImageData(0, 0, width, height)

  const prev = Date.now()
  do_fast_blur(imageData.data, width, height, radius)
  const statStr = `blur image ${width}x${height}, radius ${radius}, costs ${Date.now() - prev}ms`
  console.log(statStr)

  ctx.putImageData(imageData, 0, 0)
}

function bindSliderEvent() {
  function getCurEle() {
    return imageEle
  }
  radiusSlider.addEventListener('change', (e) => {
    const radius = e.target.value / 100 * MAX_RADIUS
    processBlur(radius, getCurEle())
  })

  videoEle.addEventListener('timeupdate', () => {
    const radius = radiusSlider.value / 100 * MAX_RADIUS
    processBlur(radius, getCurEle())
  })
}

function updateStatView(opts) {
}

videoEle.addEventListener('loadedmetadata', () => {
  init('/dist/static/fastblur_bg.wasm').then(() => {
    start()
  })
})