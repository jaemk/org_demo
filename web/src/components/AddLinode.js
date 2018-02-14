import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Button, FormGroup, FormControl, ControlLabel, HelpBlock } from 'react-bootstrap';
import axios from 'axios';


class AddLinode extends Component {
  static propTypes = {
    setError: PropTypes.func,
    closeReload: PropTypes.func,
    orgs: PropTypes.array,
  }

  constructor(props) {
    super(props);
    this.state = {
      name: '',
      orgId: null,
      submitting: false,
      valid: 'no',
    };
    this.getValidationState = this.getValidationState.bind(this);
    this.updateName = this.updateName.bind(this);
    this.updateOrg = this.updateOrg.bind(this);
    this.submit = this.submit.bind(this);
    this.checkValidity = this.checkValidity.bind(this);
  }

  checkValidity(name) {
    axios.get(`/api/exists/linode/${name}`).then(resp => {
      this.setState({valid: resp.data.exists? 'no' : 'yes'});
    }).catch(err => this.props.setError(err, 'Failed checking linode validity'));
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

  updateOrg(e) {
    let options = e.target.querySelectorAll('option');
    for (let opt of options) {
      if (opt.selected) {
        let id = parseInt(opt.value, 10);
        this.setState({orgId: id});
        return;
      }
    }
  }

  updateName(e) {
    this.setState({name: e.target.value, valid: 'unknown'});
    this.checkValidity(e.target.value);
  }

  submit(e) {
    e.preventDefault();
    if (this.state.valid !== 'yes') { return; }
    axios.post('/api/create/linode', {name: this.state.name, org_id: this.state.orgId}).then(resp => {
      this.props.closeReload();
    }).catch(err => this.props.setError(err, 'Failed creating linode'));
  }

  render() {
    return (
      <div>
        <form onSubmit={this.submit}>

          <FormGroup controlId="organizationName">
            <ControlLabel>Select Organizations</ControlLabel>
            <FormControl componentClass="select" defaultValue="capture" onChange={this.updateOrg}>
              <option disabled value="capture"> -- Select associated organizations -- </option>
              {this.props.orgs.map((org, i) =>
                <option key={i} value={org.id}>{org.name}</option>
              )}
            </FormControl>
          </FormGroup>

          <FormGroup
            controlId="linodeName"
            validationState={this.getValidationState()}
          >
            <ControlLabel>Linode Name</ControlLabel>
            <FormControl
              type="text"
              value={this.state.nanme}
              placeholder="Enter name"
              onChange={this.updateName}
            />
            <FormControl.Feedback />
            <HelpBlock>Name must be unique.</HelpBlock>
          </FormGroup>
          <Button type="submit" disabled={this.state.submitting || this.state.valid !== 'yes' || this.state.orgId === null}>
            {this.state.submitting? 'Submitting...' : 'Submit' }
          </Button>
        </form>
      </div>
    );
  }
}

export default AddLinode;

