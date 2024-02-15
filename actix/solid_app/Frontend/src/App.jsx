import { createSignal } from 'solid-js';
import styles from './App.module.css';
/** @typedef { import('solid-js').Component } Component */

/**@type Component */
const App = () => {
  const [count, setCount] = createSignal(0);
  
  return (
    <>
      <div class={styles.App}>
        <h1>Counter: {count()}</h1>
        <button onClick={() => setCount(count() + 1)}>Increment</button>
        <button onClick={() => setCount(count() - 1)}>Decrement</button>
      </div>

      <nav>
        <a href="/about">About</a>
      </nav>
    </>
  );
};

export default App;
