/**
 * A class representing the size of an item in a grid.
 */
export class ItemSize {
  /**
     * The number of rows this item spans.
     */
  rows: number

  /**
   * The number of columns this item spans.
   */
  cols: number

  /**
   * Creates a new instance of ItemSize.
   * @param {number} rows - The number of rows this item spans.
   * @param {number} cols - The number of columns this item spans.
   */
  constructor(rows: number, cols: number) {
    this.rows = rows
    this.cols = cols
  }

  /**
   * Returns a string representation of the size in the format "rows × cols", for example "2x1".
   */
  toLabel(): string {
    return `${this.rows} × ${this.cols}`
  }
}

export interface Grid<T> {
  rows: number
  cols: number
  data: T[][]
}

export type Color = [number, number, number]

export interface InferenceResult {
  probabilities: Grid<[number, Color]>
  entropy: Grid<[number, Color]>
}

export const DisplayMode = {
  Entropy: 'entropy',
  Probability: 'probability',
}
export type DisplayMode = (typeof DisplayMode)[keyof typeof DisplayMode]
