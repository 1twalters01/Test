// import {BookType} from "../types/book";
require("../types/book.js");

/** Represents a book */
export class Book {
  /** @private @type {string} title - The title of the book */
  title;
  /** @private @type {string} ISBN - The ISBN of the book */
  ISBN;
  /** @private @type {BookType} type - The type of the book */
  type;
  /** @private @type {string} author - The author of the book */
  author;
  /** @private @type {number} length - The length of the book */
  length;
  /** @private @type {string[]} genres - The list of book genres */
  genres = [];
  /** @type {number} current_page - The current page of the book */
  current_page = 0;

  /**
   * @param {string} title - The title of the book
   * @param {string} ISBN - The ISBN of the book
   * @param {string} author - The author of the book
   * @param {number} length - The length of the book
   * @param {BookType} type - The type of the book
   */
  constructor(title, ISBN, author, length, type=BookType.BOOK) {
    /** @private */
    this.title = String(title).split(" ").map(word => word[0].toUpperCase() + word.substring(1)).join(" ");
    /** @private */
    let valid = this.validateISBN(ISBN);
    if (valid) {
      this.ISBN = ISBN;
    } else {
      this.ISBN = "";
      console.error("invalid ISBN")
    }
    /** @private */
    this.type = BookType.EBOOK;
    /** @private */
    this.author = String(author).split(" ").map(word => word[0].toUpperCase() + word.substring(1)).join(" ");
    /** @private */
    this.length = length;
  }

  /** @param {string} ISBN */
  validateISBN(ISBN) {
      console.log("ISBN: " + ISBN);
      return true;
  }

  /** @param {...string} genres - The list of book genres */
  selectGenres() {
    let genres = Array.prototype.slice.call(arguments);

    for (let i = 0; i < genres.length; i++) {
      genres[i] = String(genres[i]).split(" ").map(word => word[0].toUpperCase() + word.substring(1)).join(" ");
    }

    this.genres = genres;
  }

  /** @param {...string} genres - The list of book genres */
  appendGenres() {
    let genres = this.genres.concat(Array.prototype.slice.call(arguments));

    for (let i = 0; i < genres.length; i++) {
      genres[i] = String(genres[i]).split(" ").map(word => word[0].toUpperCase() + word.substring(1)).join(" ");
    }

    this.genres = genres;

  }

  /** @param {...string} genres - The list of book genres */
  deleteGenres() {
    let arg_array = Array.prototype.slice.call(arguments);

    for (let i = 0; i < arg_array.length; i++) {
      let index = this.genres.indexOf(arg_array[i]);
      if (index !== -1) {
        this.genres.splice(index, 1);
      }
    }

    for (let i = 0; i < this.genres.length; i++) {
      this.genres[i] = String(this.genres[i]).split(" ").map(word => word[0].toUpperCase() + word.substring(1)).join(" ");
    }

    this.genres = this.genres;

  }

  nextPage() {
    this.current_page++;
  }

  previousPage() {
    this.current_page--;
  }

  /** @param {number} page - The page to go to */
  goToPage(page) {
    if (page < 0) {
      this.current_page = 0;
    } else if (page <= this.length) {
      this.current_page = page;
    } else {
      this.current_page = this.length;
    }
  }

  /** @return {string} */
  getTitle() {
    return this.title;
  }

  /** @return {string} */
  getISBN() {
    return this.ISBN;
  }

  /** @return {BookType} */
  getBookType() {
    return this.type;
  }

  /** @return {string} */
  getAuthor() {
    return this.author;
  }

  /** @return {number} */
  getLength() {
    return this.length;
  }
}
