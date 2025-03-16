import { defineStore } from 'pinia'

export const useItemStore = defineStore('item', () => {
  const settings = ref([
    { size: { cols: 2, rows: 1 }, count: 0 },
    { size: { cols: 2, rows: 1 }, count: 0 },
    { size: { cols: 3, rows: 2 }, count: 0 },
    { size: { cols: 3, rows: 3 }, count: 0 },
  ],
  )
  function getSize(index: number) {
    return settings.value[index].size
  }
  function getCount(index: number) {
    return settings.value[index].count
  }

  function setSize(index: number, size: ItemSize) {
    settings.value[index].size = size
  }
  function setCount(index: number, count: number) {
    settings.value[index].count = count
  }
  function toRectangleArray() {
    const result = []

    for (let i = 0; i < settings.value.length; i++) {
      const item = settings.value[i]
      for (let j = 0; j < item.count; j++) {
        result.push({ width: item.size.cols, height: item.size.rows })
      }
    }

    return result
  }

  return {
    settings,
    getSize,
    getCount,
    setSize,
    setCount,
    toRectangleArray,
  }
})

export const useGridStore = defineStore('grid', () => {
  const _size = ref(new ItemSize(5, 9))
  const mask = ref(Array.from({ length: _size.value.rows }, () => Array(_size.value.cols).fill(false)))

  const size = computed(() => {
    return new ItemSize(_size.value.rows, _size.value.cols)
  })
  function flipMask(x: number, y: number) {
    if (y < 0 || y >= _size.value.rows || x < 0 || x >= _size.value.cols) {
      return
    }
    mask.value[y][x] = !mask.value[y][x]
  }
  function reset() {
    mask.value = Array.from({ length: _size.value.rows }, () => Array(_size.value.cols).fill(false))
  }
  function isAllMasked() {
    for (let i = 0; i < _size.value.rows; i++) {
      for (let j = 0; j < _size.value.cols; j++) {
        if (mask.value[i][j]) {
          return false
        }
      }
    }
    return true
  }

  return {
    size,
    mask,
    flipMask,
    reset,
    isAllMasked,
  }
})

export const useDisplayModeStore = defineStore('displayMode', () => {
  const _mode = ref(DisplayMode.Entropy)

  const mode = computed<typeof DisplayMode[keyof typeof DisplayMode]>(() => {
    return _mode.value
  })

  function setMode(newMode: typeof DisplayMode[keyof typeof DisplayMode]) {
    _mode.value = newMode
  }

  return {
    mode,
    setMode,
  }
})
