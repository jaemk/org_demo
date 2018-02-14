import React, { Component } from 'react';
import PropTypes from 'prop-types';
import axios from 'axios';
import { Button } from 'react-bootstrap';
import UserDetails from './components/UserDetails';
import OrgDetails from './components/OrgDetails';
import MyModal from './components/MyModal';
import AddOrg from './components/AddOrg';
import AddUser from './components/AddUser';
import AddLinode from './components/AddLinode';


class App extends Component {
  static propsTypes = {
    setError: PropTypes.func.isRequired,
  }

  constructor(props) {
    super(props)
    this.state = {
      orgs: null,
      selectedUserId: null,
      addingOrg: false,
      addingUser: false,
      addingLinode: false,

      broke: false,
      error: null,
      errorMsg: ''
    };
    this.fetchOrgs = this.fetchOrgs.bind(this);
    this.selectUser = this.selectUser.bind(this);
    this.unselectUser = this.unselectUser.bind(this);
    this.addOrg = this.addOrg.bind(this);
    this.addUser = this.addUser.bind(this);
    this.addLinode = this.addLinode.bind(this);
    this.setError = this.setError.bind(this);
    this.closeReload = this.closeReload.bind(this);
  }

  componentWillMount() {
    this.fetchOrgs();
  }

  fetchOrgs() {
    axios.get('/api/orgs').then(resp => {
      this.setState({orgs: resp.data.orgs})
    }).catch(err => this.setError(err, 'Failed fetching orgs'));
  }

  selectUser(userId) {
    this.setState({selectedUserId: userId});
  }

  unselectUser() {
    this.setState({selectedUserId: null});
  }

  setError(err, msg) {
    console.error(err, msg);
    this.setState({
      selectedUserId: null,
      addingOrg: false,
      addingUser: false,
      addingLinode: false,
      broke: true, error: err, errorMsg: msg
    });
  }

  closeReload(action) {
    switch (action) {
      case 'addingOrg':
        this.setState({addingOrg: false});
        break;
      case 'addingUser':
        this.setState({addingUser: false});
        break;
      case 'addingLinode':
        this.setState({addingLinode: false});
        break;
      default:
        console.error(`Invalid action: ${action}`)
    }
    this.fetchOrgs();
  }

  addOrg() {
    this.setState({
      addingOrg: true,
    });
  }

  addUser(orgIds) {
    this.setState({
      addingUser: true,
    });
  }

  addLinode(orgId) {
    this.setState({
      addingLinode: true,
    });
  }

  render() {
    const orgsExist = this.state.orgs && this.state.orgs.length > 0;
    return (
      <div style={{paddingLeft: '20px'}}>
        <Button style={{margin: '5px'}} onTouchTap={() => this.addOrg()}>Add Org</Button>
        <Button style={{margin: '5px'}} disabled={!orgsExist} onTouchTap={() => this.addUser()}>Add User</Button>
        <Button style={{margin: '5px'}} disabled={!orgsExist} onTouchTap={() => this.addLinode()}>Add Linode</Button>

        {/* Org Details List */}
        {!orgsExist?
            <div style={{margin: '10px'}}>
              Add an organization to get started!
            </div>
            :
          this.state.orgs.map(org => {
            return (
              <div key={org.id} style={{marginBottom: '20px'}}>
                <OrgDetails org={org} selectUser={this.selectUser}/>
              </div>
            );
        })}

        {/* User Detail Modal */}
        <MyModal
          isVisible={this.state.selectedUserId !== null}
          close={this.unselectUser}
          closeable={true}
          title="User details"
          body={() =>
              <UserDetails
                userId={this.state.selectedUserId}
                setError={this.setError}
              />
          }
        />

        {/* Add Org Modal */}
        <MyModal
          isVisible={this.state.addingOrg}
          close={()=>this.setState({addingOrg: false})}
          closeable={true}
          title="Add Organization"
          body={() =>
              <AddOrg
                setError={this.setError}
                closeReload={() => this.closeReload('addingOrg')}
              />
          }
        />

        {/* Add User Modal */}
        <MyModal
          isVisible={this.state.addingUser}
          close={()=>this.setState({addingUser: false})}
          closeable={true}
          title="Add user"
          body={() =>
              <AddUser
                orgs={this.state.orgs}
                setError={this.setError}
                closeReload={() => this.closeReload('addingUser')}
              />
          }
        />

        {/* Add Linode Modal */}
        <MyModal
          isVisible={this.state.addingLinode}
          close={()=>this.setState({addingLinode: false})}
          closeable={true}
          title="Add linode"
          body={() =>
              <AddLinode
                orgs={this.state.orgs}
                setError={this.setError}
                closeReload={() => this.closeReload('addingLinode')}
              />
          }
        />

        {/* Error Modal */}
        <MyModal
          isVisible={this.state.broke}
          close={()=>{}}
          closeable={false}
          title="Something went wrong..."
          body={() =>
            <div>
              <div style={{margin: '10px 0px'}}>
                {this.state.errorMsg}
              </div>
              <div style={{fontWeight: 'bold', marginBottom: '5px'}}>
                Try refreshing the page
              </div>
              <Button onTouchTap={() => window.location.reload()}> Refresh </Button>
            </div>
          }
        />
      </div>
    );
  }
}

export default App;

