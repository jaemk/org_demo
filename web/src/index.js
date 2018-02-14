import React from 'react';
import ReactDOM from 'react-dom';
import injectTapEventPlugin from 'react-tap-event-plugin';
import { Router, Route } from 'react-router-dom';
import { createHashHistory } from 'history';
import './index.css';
import Header from './components/Header';
import App from './App';

injectTapEventPlugin();


const history = createHashHistory({
  basename: '',
  hashType: 'slash',
});

const style = {
  padding: '10px 30px',
};


ReactDOM.render(
  <Router history={history}>
    <div style={style}>
      <Header/>
      <Route path="/" component={App}/>
    </div>
  </Router>,
  document.getElementById('root'));

