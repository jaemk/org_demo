import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Well } from 'react-bootstrap';
import styles from '../styles';


class OrgDetails extends Component {
  static propTypes = {
    org: PropTypes.object,
    selectUse: PropTypes.func,
  }

  render() {
    const orgName = {fontSize: '20px', fontWeight: 500};

    return (
      <Well bsSize="large">
        <div style={styles.table}>
          <div style={styles.row}>
            <span style={orgName}>
              {this.props.org.name}
            </span>
          </div>

          <div style={styles.row}>
            <div style={styles.titleCell}>
              <span style={styles.titleSpan}>
                Users:
              </span>
            </div>
          </div>
          {this.props.org.users.length === 0?
            <div style={styles.row}>
              <div style={styles.itemCell}>
                No Users
              </div>
            </div>
              :
            this.props.org.users.map((user, i) =>
              <div key={i} style={styles.row}>
                <div style={styles.itemCell}>
                  <a href="#/" onTouchTap={() => this.props.selectUser(user.id)}>{user.email}</a>
                </div>
              </div>
            )
          }

          <div style={styles.row}>
            <div style={styles.titleCell}>
              <span style={styles.titleSpan}>
                Linodes:
              </span>
            </div>
          </div>
          {this.props.org.linodes.length === 0?
            <div style={styles.row}>
              <div style={styles.itemCell}>
                No Linodes
              </div>
            </div>
              :
            this.props.org.linodes.map((linode, i) =>
              <div key={i} style={styles.row}>
                <div style={styles.itemCell}>
                  {linode.name}
                </div>
              </div>
            )
          }
        </div>
      </Well>
    );
  }
}

export default OrgDetails;

