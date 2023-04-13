/* Enable mode to accept txt files as mentioned in the task */
.mode tabs
.separator "\t"
CREATE TABLE db('sex' TEXT, 'heart2' INTEGER, 'agedx' INTEGER, 'age_allevts' INTEGER, 'outcome' INTEGER, 'outcome2' INTEGER);

/* Modify the path as per the location of the input file */
.import C:/Users/akhil/Downloads/testdata.txt db

/* Query to count number of individuals who are identified as males and have agedx >= 10 */
SELECT COUNT(sex) FROM db WHERE agedx >= 10 AND sex  = 'Male';




