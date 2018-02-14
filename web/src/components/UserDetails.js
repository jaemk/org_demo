import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Well } from 'react-bootstrap';
import axios from 'axios';
import styles from '../styles';


class UserDetails extends Component {
  static propTypes = {
    userId: PropTypes.number,
    setError: PropTypes.func,
  }

  constructor(props) {
    super(props);
    this.state = {
      user: null,
    }
  }

  componentWillMount() {
    if (!this.props.userId) { return; }
    axios.get(`/api/user/${this.props.userId}`).then(resp => {
      this.setState({user: resp.data.user});
    }).catch(err => this.props.setError(err, 'failed fetching user details'));
  }

  render() {
    const getOrgNameById = (id) => {
      for (let org of this.state.user.orgs) {
        if (org.id === id) { return org.name; }
      }
    };

    return (
      <div>
        {this.state.user === null? 'Loading...' :
          <div>
            <div style={{marginBottom: '30px'}}>
              <b>Email:</b> {this.state.user.email}
            </div>

            <Well bsSize="large">
              <div style={styles.table}>
                <div style={styles.row}>
                  <div style={styles.titleCell}>
                    <span style={styles.titleSpan}>
                      Organizations:
                    </span>
                  </div>
                </div>
                {this.state.user.orgs.length === 0?
                  <div style={styles.row}>
                    <div style={styles.itemCell}>
                      No associated organizations
                    </div>
                  </div>
                    :
                  this.state.user.orgs.map((org, i) =>
                    <div key={i} style={styles.row}>
                      <div style={styles.itemCell}>
                        {org.name}
                      </div>
                    </div>
                  )
                }
              </div>
            </Well>

            <Well bsSize="large">
              <div style={styles.table}>
                <div style={styles.row}>
                  <div style={styles.titleCell}>
                    <span style={styles.titleSpan}>
                      Linodes:
                    </span>
                  </div>
                </div>
                {this.state.user.linodes.length === 0?
                  <div style={styles.row}>
                    <div style={styles.itemCell}>
                      No associated linodes
                    </div>
                  </div>
                    :
                  this.state.user.linodes.map((linode, i) =>
                    <div key={i} style={styles.row}>
                      <div style={styles.itemCell}>
                        {linode.name} ({getOrgNameById(linode.org)})
                      </div>
                    </div>
                  )
                }
              </div>
            </Well>
          </div>
        }
      </div>
    );
  }
}

export default UserDetails;

