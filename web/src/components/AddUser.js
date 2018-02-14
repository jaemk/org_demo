import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Button, FormGroup, FormControl, ControlLabel, HelpBlock } from 'react-bootstrap';
import axios from 'axios';


class AddOrg extends Component {
  static propTypes = {
    setError: PropTypes.func,
    closeReload: PropTypes.func,
    orgs: PropTypes.array,
  }

  constructor(props) {
    super(props);
    this.state = {
      email: '',
      orgIds: [],
      submitting: false,
      valid: 'no',
    };
    this.getValidationState = this.getValidationState.bind(this);
    this.updateEmail = this.updateEmail.bind(this);
    this.updateOrg = this.updateOrg.bind(this);
    this.submit = this.submit.bind(this);
    this.checkValidity = this.checkValidity.bind(this);
  }

  checkValidity(email) {
    axios.get(`/api/exists/user/${email}`).then(resp => {
      this.setState({valid: resp.data.exists? 'no' : 'yes'});
    }).catch(err => this.props.setError(err, 'Failed checking user validity'));
  }

  getValidationState() {
    if (this.state.email.length === 0) { return; }
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

  updateOrg(e) {
    let options = e.target.querySelectorAll('option');
    let selectedOrgs = [];
    for (let opt of options) {
      if (opt.selected) {
        selectedOrgs.push(parseInt(opt.value, 10));
      }
    }
    this.setState({orgIds: selectedOrgs});
  }

  updateEmail(e) {
    this.setState({email: e.target.value, valid: 'unknown'});
    this.checkValidity(e.target.value);
  }

  submit(e) {
    e.preventDefault();
    if (this.state.valid !== 'yes') { return; }
    axios.post('/api/create/user', {email: this.state.email, org_ids: this.state.orgIds}).then(resp => {
      this.props.closeReload();
    }).catch(err => this.props.setError(err, 'Failed creating user'));
  }

  render() {
    return (
      <div>
        <form onSubmit={this.submit}>

          <FormGroup controlId="organizationName">
            <ControlLabel>Select Organizations</ControlLabel>
            <FormControl componentClass="select" onChange={this.updateOrg} multiple defaultValue={["capture"]}>
              <option disabled value="capture"> -- Select associated organizations -- </option>
              {this.props.orgs.map((org, i) =>
                <option key={i} value={org.id}>{org.name}</option>
              )}
            </FormControl>
          </FormGroup>

          <FormGroup
            controlId="userEmail"
            validationState={this.getValidationState()}
          >
            <ControlLabel>User Email</ControlLabel>
            <FormControl
              type="text"
              value={this.state.email}
              placeholder="Enter email"
              onChange={this.updateEmail}
            />
            <FormControl.Feedback />
            <HelpBlock>Email must be unique.</HelpBlock>
          </FormGroup>
          <Button type="submit" disabled={this.state.submitting || this.state.valid !== 'yes' || this.state.orgIds.length === 0}>
            {this.state.submitting? 'Submitting...' : 'Submit' }
          </Button>
        </form>
      </div>
    );
  }
}

export default AddOrg;

