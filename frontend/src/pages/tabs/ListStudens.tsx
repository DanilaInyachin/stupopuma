import { Box, List, ListItem, ListItemText, Typography } from '@mui/material';
import React, { useContext, useEffect, useState } from 'react';
import CurrentUserContext from '../../context';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

interface IListStudents {
  course_name: string;
  students: string[];
}

const initialState: IListStudents = {
  course_name: '',
  students: [],
};

const ListStudens = () => {
  const [listsStudents, setListsStudents] = useState<IListStudents[]>([
    initialState,
  ]);
  const context = useContext(CurrentUserContext);
  const navigate = useNavigate();

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .post('//localhost:8080/enrolled_courses_list', {
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          setListsStudents(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, setListsStudents]);

  return (
    <Box>
      <Typography variant="h6" component="h2">
        List of students
      </Typography>
      <Typography variant="body1">
        Here is the list of all pending list of students.
      </Typography>
      <List>
        {listsStudents.map((list, index) => (
          <ListItem key={index} divider>
            <Typography variant="body1">
              {list.course_name}
            </Typography>
            <List>
              {list.students.map((student, index) => (
                <ListItem key={index} divider >
                  <ListItemText primary={student} />
                </ListItem>
              ))}
            </List>
          </ListItem>
        ))}
      </List>
    </Box>
  );
};

export default ListStudens;
