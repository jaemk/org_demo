import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Button, Modal } from 'react-bootstrap';


class MyModal extends Component {
  static propTypes = {
    isVisible: PropTypes.bool,
    closeable: PropTypes.bool,
    close: PropTypes.func,
    title: PropTypes.string,
    body: PropTypes.func,
  }

  render() {
    return (
      <Modal show={this.props.isVisible} onHide={this.props.close}>
        <Modal.Header closeButton={this.props.closeable}>
          <Modal.Title>
            {this.props.title}
          </Modal.Title>
        </Modal.Header>
        <Modal.Body>
          {this.props.body()}
        </Modal.Body>
        {this.props.closeable?
        <Modal.Footer>
          <Button onTouchTap={this.props.close}> Close </Button>
        </Modal.Footer>
        : ''}
      </Modal>
    );
  }
}

export default MyModal;

