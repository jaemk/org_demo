import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Button, FormGroup, FormControl, ControlLabel, HelpBlock } from 'react-bootstrap';
import axios from 'axios';
import debounce from 'lodash/debounce';


class AddOrg extends Component {
  static propTypes = {
    setError: PropTypes.func,
    closeReload: PropTypes.func,
  }

  constructor(props) {
    super(props);
    this.state = {
      name: '',
      submitting: false,
      valid: 'no',
    };
    this.getValidationState = this.getValidationState.bind(this);
    this.update = this.update.bind(this);
    this.submit = this.submit.bind(this);
    this._checkValidity = this._checkValidity.bind(this);
    this.checkValidity = debounce(this._checkValidity, 250);
  }

  _checkValidity() {
    if (!this.state.name) { return; }
    axios.get(`/api/exists/org/${this.state.name}`).then(resp => {
      this.setState({valid: resp.data.exists? 'no' : 'yes'});
    }).catch(err => this.props.setError(err, 'Failed checking org validity'));
  }

  getValidationState() {
    if (this.state.name.length === 0) { return; }
    switch (this.state.valid) {
      case 'yes':
        return 'success';
      case 'no':
        return 'error';
      case 'unknown':
        return 'success';
      default:
        return 'error';
    }
  }

  update(e) {
    this.setState({name: e.target.value, valid: 'unknown'});
    this.checkValidity(e.target.value);
  }

  submit(e) {
    e.preventDefault();
    if (this.state.valid !== 'yes') { return; }
    axios.post('/api/create/org', {name: this.state.name}).then(resp => {
      this.props.closeReload();
    }).catch(err => this.props.setError(err, 'Failed creating organization'));
  }

  render() {
    return (
      <div>
        <form onSubmit={this.submit}>
          <FormGroup
            controlId="organizationName"
            validationState={this.getValidationState()}
          >
            <ControlLabel>Organization Name</ControlLabel>
            <FormControl
              type="text"
              value={this.state.name}
              placeholder="Enter name"
              onChange={this.update}
            />
            <FormControl.Feedback />
            <HelpBlock>Name must be unique.</HelpBlock>
          </FormGroup>
          <Button type="submit" disabled={this.state.submitting || this.state.valid !== 'yes'}>
            {this.state.submitting? 'Submitting...' : 'Submit' }
          </Button>
        </form>
      </div>
    );
  }
}

export default AddOrg;

