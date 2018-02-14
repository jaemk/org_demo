import React from 'react';
import { Navbar } from 'react-bootstrap';


const Header = () => {
  return (
    <Navbar fluid={true}>
      <Navbar.Header>
        <Navbar.Brand style={{marginLeft: 0}}>
          <a href="#/">OrgDemo</a>
        </Navbar.Brand>
      </Navbar.Header>
    </Navbar>
  );
};

export default Header;

