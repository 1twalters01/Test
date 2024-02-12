
/** 
  * @enum {number}
  */
export const CoverEnum = {
  /** @type {1} */
  HARDBACK: 1,
  /** @type {2} */
  PAPERBACK: 2
}

/**
    * @readonly
    * @typedef {object} CoverType
    * @property {CoverEnum} coverEnum
    */

/**
  * @readonly
  * @typedef {Object} BookMetadata
  * @property {CoverType} coverType
  */

/** @type {BookMetadata} */
const bookMetadata = {
  coverType: CoverEnum.HARDBACK,
}

/**
  * @readonly
  * @enum {(BookMetadata|"cd"|"video"|"ebook"|"magazine"|"other")} BookType
  */
export const BookType = {
  /** @type {BookMetadata} */
  BOOK: bookMetadata,
  /** @type {"cd"} */
  CD: "cd",
  /** @type {"video"} */
  VIDEO: "video",
  /** @type {"ebook"} */
  EBOOK: "ebook",
  /** @type {"magazine"} */
  MAGAZINE: "magazine",
  /** @type {"other"} */
  OTHER: "other"
}

