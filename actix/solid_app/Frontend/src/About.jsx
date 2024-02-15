import styles from './App.module.css';
/** @typedef { import('solid-js').Component } Component */

/** @type { Component } */
const About = () => {
  return (
    <div class={styles.About}>
      <h1>About</h1>
      <p>This is the about page.</p>
    </div>
  );
};

export default About;

