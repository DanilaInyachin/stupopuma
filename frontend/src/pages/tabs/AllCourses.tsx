import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box, Button, TextField } from '@mui/material';
import axios from 'axios';
import ElementCourse from '../../components/ElementCourse';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../context';

type Role = 'Администратор' | 'Преподаватель' | 'Ученик';

interface AllCourseProps {
  needButton?: boolean;
}

const AllCourses: FC<AllCourseProps> = ({ needButton = true }) => {
  const [namecourses, setNamecourses] = useState<string[]>([]);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);
  const [role, setRole] = useState<Role>('Ученик');
  const [newCourseName, setNewCourseName] = useState('');
  const [addingCourse, setAddingCourse] = useState(false);

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .post('//localhost:8080/view_user', {
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          setRole(response.data.role);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
      axios
        .get('//localhost:8080/view_courses')
        .then((response) => {
          console.log(response);
          setNamecourses(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, setNamecourses]);

  const handleAddCourse = () => {
    if (context && context.isAuthAndToken) {
      // axios
      //   .post('//localhost:8080/add_course', {
      //     nameCourses: newCourseName,
      //     token: context.isAuthAndToken,
      //   })
      //   .then((response) => {
      //     console.log(response);
      //     setNewCourseName('');
      //     setAddingCourse(false);
      //     setNamecourses((prev) => [...prev, newCourseName]); // Добавляем новый курс в локальный state
      //   })
      //   .catch((error) => {
      //     console.error('Error: ', error);
      //   });
      console.log("NEW Course: ", newCourseName);
    }
  };

  return (
    <Box>
      <Typography variant="h6" component="h2">
        All Courses
      </Typography>
      <Typography variant="body1">
        Here is the list of all available courses.
      </Typography>
      {namecourses.map((course) => (
        <ElementCourse
          key={course}
          namecourse={course}
          needButton={needButton}
          isTeacher={role === 'Преподаватель'}
        />
      ))}
      {role === 'Преподаватель' && (
        <Box sx={{ mt: 2 }}>
          {!addingCourse ? (
            <Button
              variant="outlined"
              color="primary"
              onClick={() => setAddingCourse(true)}
            >
              Добавить новый курс
            </Button>
          ) : (
            <Box sx={{ display: 'flex', flexDirection: 'column', mt: 2 }}>
              <TextField
                label="Название нового курса"
                variant="outlined"
                value={newCourseName}
                onChange={(e) => setNewCourseName(e.target.value)}
                onKeyPress={(e) => {
                  if (e.key === 'Enter') handleAddCourse();
                }}
              />
              <Button
                variant="contained"
                color="primary"
                onClick={handleAddCourse}
                sx={{ mt: 2 }}
              >
                Enter
              </Button>
            </Box>
          )}
        </Box>
      )}
    </Box>
  );
};

export default AllCourses;
