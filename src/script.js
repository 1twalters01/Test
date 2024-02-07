import { Book } from './classes/book.js';

/**
 * @type {number}
 */

/** @type {string} */
let title = 'Performance';
/** @type {string} */
let ISBN = '978-1-4412-1488-0';
/** @type {string} */
let author = 'Tyler Walters';

let valid_book = new Book(title, ISBN, author, 600);
valid_book.selectGenres('computer science', 'Self Help');
console.log(valid_book);
// new Book(2, 'he', 900);

valid_book.selectGenres();
valid_book.nextPage();
console.log(valid_book);

valid_book.appendGenres('Computer Science', 'Self Help');
valid_book.previousPage();
console.log(valid_book);

valid_book.deleteGenres('Computer Science', 'wrong val');
valid_book.goToPage(1300);
console.log(valid_book);
