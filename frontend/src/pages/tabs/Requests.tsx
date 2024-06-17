import React, { FC, useContext, useEffect, useState } from 'react';
import {
  Typography,
  Box,
  Button,
  List,
  ListItem,
  ListItemText,
} from '@mui/material';
import axios from 'axios';
import CurrentUserContext from '../../context';
import { useNavigate } from 'react-router-dom';

// interface IRequest {
//   id: number;
//   studentName: string;
//   courseName: string;
// }

interface IRequest {
  mail: string;
  surname: string;
  firstname: string;
  lastname: string;
  course_name: string;
}

const Requests: FC = () => {
  const [requests, setRequests] = useState<IRequest[]>([]);
  const context = useContext(CurrentUserContext);
  const navigate = useNavigate();

  const fetchRequests = () => {
    axios
      .post('//localhost:8080/unenrolled_courses', {
        token: context?.isAuthAndToken,
      })
      .then((response) => {
        setRequests(response.data);
      })
      .catch((error) => {
        console.error('Error: ', error);
      });
  };

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      fetchRequests();
    }
  }, [context, navigate]);

  const handleConfirm = (request: IRequest) => {
    if (context && context.isAuthAndToken) {
      axios
        .put('//localhost:8080/change_user_enrollment', {
          mail: request.mail,
          course_name: request.course_name,
          enrollment: true,
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          fetchRequests();
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  };

  return (
    <Box>
      <Typography variant="h6" component="h2">
        Requests
      </Typography>
      <Typography variant="body1">
        Here is the list of all pending course requests.
      </Typography>
      <List>
        {requests.map((request, index) => (
          <ListItem key={index} divider>
            <ListItemText
              primary={
                request.lastname +
                ' ' +
                request.firstname +
                ' ' +
                request.surname
              }
              secondary={request.course_name}
            />
            <Button
              variant="contained"
              color="primary"
              onClick={() => handleConfirm(request)}
            >
              Confirm
            </Button>
          </ListItem>
        ))}
      </List>
    </Box>
  );
};

export default Requests;
